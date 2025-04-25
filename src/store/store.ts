import { Action, configureStore, ThunkAction } from '@reduxjs/toolkit'
import { wasmSlice } from './slices/slice.wasm'
import { TypedUseSelectorHook, useDispatch, useSelector } from 'react-redux'

export const store = configureStore({
  reducer: {
		wasm: wasmSlice.reducer,
	},
	middleware: (getDefaultMiddleware) =>   
		getDefaultMiddleware({  
			serializableCheck: false // Отключает проверку сериализуемости   
		  }).concat(), // Если нужно добавить дополнительные middleware, делайте это здесь  
	// enhancers: [], 
	devTools: true,  

})
// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch

export type AppThunk<ReturnType = void> = ThunkAction<
  ReturnType,
  RootState,
  unknown,
  Action<string>
>;
export const useAppDispatch: () => AppDispatch = useDispatch;
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector;