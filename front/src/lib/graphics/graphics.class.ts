export class Graphics {
	static drawMinimap(
		canvas: HTMLCanvasElement,
		indexedPixels: Vec<u8>,
		palette: Vec<u8>,
		width: number,
		height: number
	) {
		const ctx = canvas.getContext('2d');
		if (!ctx) {
			console.error('Failed to get canvas context for minimap');
			return;
		}

		canvas.width = width;
		canvas.height = height;
		ctx.clearRect(0, 0, canvas.width, canvas.height);
		const imageData = ctx.createImageData(width, height);
		const data = imageData.data;
		for (let i = 0; i < indexedPixels.length; i++) {
			const index = indexedPixels[i];
			const colorOffset = index * 3;
			data[i * 4] = palette[colorOffset];
			data[i * 4 + 1] = palette[colorOffset + 1];
			data[i * 4 + 2] = palette[colorOffset + 2];
			data[i * 4 + 3] = 0xff;
		}
		ctx.putImageData(imageData, 0, 0);
	}

	static drawPixels(
		canvas: HTMLCanvasElement,
		pixelData: Vec<u8>,
		width: number,
		height: number,
		targetWidth?: number,
		targetHeight?: number,
	) {
		let useTargetWidth = targetWidth ?? width;
		let useTargetHeight = targetHeight ?? height;

		const ctx = canvas.getContext('2d');
		if (!ctx) {
			console.error('Failed to get canvas context for drawing pixels');
			return;
		}

		const imageData = ctx.createImageData(width, height);
		imageData.data.set(pixelData);

		if (useTargetHeight === height && useTargetWidth === width) {
			canvas.width = width;
			canvas.height = height;
			ctx.putImageData(imageData, 0, 0);
		} else {
			const offscreenCanvas = new OffscreenCanvas(width, height);
			const offscreenCtx = offscreenCanvas.getContext('2d');
			if (!offscreenCtx) {
				console.error('Failed to get offscreen canvas context');
				return;
			}
			offscreenCtx.putImageData(imageData, 0, 0);
			canvas.width = useTargetWidth;
			canvas.height = useTargetHeight;
			ctx.drawImage(offscreenCanvas, 0, 0, width, height, 0, 0, useTargetWidth, useTargetHeight);
		}
	}

	static clear(canvas: HTMLCanvasElement, color?: string) {
		const ctx = canvas.getContext('2d');
		if (!ctx) {
			console.error('Failed to get canvas context for clearing');
			return;
		}
		if (color) {
			ctx.fillStyle = color;
			ctx.fillRect(0, 0, canvas.width, canvas.height);
		} else {
			ctx.clearRect(0, 0, canvas.width, canvas.height);
		}
	}
}
