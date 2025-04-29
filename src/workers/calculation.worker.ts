// Worker для обработки вычислений
const ctx: Worker = self ;

ctx.addEventListener('message', async (event) => {
  try {
    // Импортируем необходимые функции
    const { transformMainWDT_To_Order_Z } = await import('../helpers/transformMainWDT_ToOrder_Z.ts');
    const result = transformMainWDT_To_Order_Z(event.data);
    ctx.postMessage({ status: 'success', result });
  } catch (error) {
    ctx.postMessage({ status: 'error', error: error.message });
  }
});

export default {} as typeof Worker & { new (): Worker };