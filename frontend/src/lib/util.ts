export function formatDate(timestamp: number) {
	const date = new Date(timestamp * 1000);
	return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
}

export function truncateContent(content: string, maxLength: number = 200) {
	if (content.length <= maxLength) return content;
	return content.substring(0, maxLength) + '...';
}

export function formatFileSize(size: number) {
	return size < 1024
		? size + 'B'
		: size < 1024 * 1024
			? (size / 1024).toFixed(2) + 'KB'
			: (size / 1024 / 1024).toFixed(2) + 'MB';
}
