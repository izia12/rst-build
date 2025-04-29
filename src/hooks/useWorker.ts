import { useCallback, useEffect, useRef, useState } from 'react';

type WorkerStatus = 'idle' | 'loading' | 'error';

export const useWorker = <T extends (...args: unknown[]) => unknown>(fn: T) => {
  const workerRef = useRef<Worker>();
  const [status, setStatus] = useState<WorkerStatus>('idle');
  const [error, setError] = useState<string | null>(null);
  const abortControllerRef = useRef<AbortController>();

  const runTask = useCallback((...args: Parameters<T>): Promise<ReturnType<T>> => {
    return new Promise((resolve, reject) => {
      if (typeof window === 'undefined') {
        reject(new Error('Web Workers доступны только в браузере'));
        return;
      }

      abortControllerRef.current = new AbortController();
      setStatus('loading');
      setError(null);

      const workerCode = `
        importScripts('${window.location.origin}/transformLinesPointsIntoArray.js');
        self.onmessage = async (event) => {
          const fn = ${fn.toString()};
          try {
            const result = await fn(...event.data.args);
            self.postMessage({ 
              status: 'success', 
              result 
            });
          } catch (error) {
            self.postMessage({ 
              status: 'error', 
              error: error.message 
            });
          }
        };
      `;

      const blob = new Blob([workerCode], { type: 'application/javascript' });
      workerRef.current = new Worker(URL.createObjectURL(blob));

      workerRef.current.onmessage = (event: MessageEvent) => {
        if (abortControllerRef.current?.signal.aborted) return;

        if (event.data.status === 'success') {
          resolve(event.data.result);
          setStatus('idle');
        } else {
          reject(new Error(event.data.error));
          setError(event.data.error);
          setStatus('error');
        }
        workerRef.current?.terminate();
      };

      workerRef.current.postMessage({ args });
    });
  }, [fn]);

  const abort = useCallback(() => {
    abortControllerRef.current?.abort();
    workerRef.current?.terminate();
    setStatus('idle');
  }, []);

  useEffect(() => {
    return () => {
      abort();
    };
  }, [abort]);

  return { runTask, status, error, abort };
};