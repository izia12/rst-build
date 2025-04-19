export function getRandomColor():string {
	const randomColor = Math.floor(Math.random() * 16777215).toString(16);  
    // Добавление '#' в начало и дополнение до 6 символов  
    return `#${randomColor.padStart(6, '0')}`;
}