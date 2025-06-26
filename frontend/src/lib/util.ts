export function formatDate(timestamp: number) {
	const date = new Date(timestamp * 1000);
	return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
}

export function truncateContent(content: string, maxLength: number = 200) {
	if (content.length <= maxLength) return content;
	return content.substring(0, maxLength) + '...';
}