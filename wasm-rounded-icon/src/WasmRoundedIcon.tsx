import React, { useState, useEffect, ChangeEvent } from 'react';
import init, { add_rounded_corners } from './pkg/wasm_rounded_icon';

const WasmRoundedIcon: React.FC = () => {
    const [imageSrc, setImageSrc] = useState<string | null>(null);
    const [roundedImageSrc, setRoundedImageSrc] = useState<string | null>(null);
    const [radius, setRadius] = useState<number>(20); // 默认圆角半径为20像素

    useEffect(() => {
        init()
            .then(() => console.log('WASM module initialized'))
            .catch(console.error);
    }, []);

    const handleFileChange = (event: ChangeEvent<HTMLInputElement>) => {
        const file = event.target.files?.[0];
        if (!file) return;

        const reader = new FileReader();
        reader.onload = (e) => {
            const src = e.target?.result as string;
            setImageSrc(src);
            generateRoundedImage(src, radius);
        };
        reader.readAsDataURL(file);
    };

    const handleRadiusChange = (event: ChangeEvent<HTMLInputElement>) => {
        const newRadius = Number(event.target.value);
        setRadius(newRadius);
        if (imageSrc) {
            generateRoundedImage(imageSrc, newRadius);
        }
    };

    const generateRoundedImage = async (src: string, radius: number) => {
        const img = new Image();
        img.onload = async () => {
            const canvas = document.createElement('canvas');
            canvas.width = img.width;
            canvas.height = img.height;
            const ctx = canvas.getContext('2d');
            if (!ctx) return;

            ctx.drawImage(img, 0, 0);

            canvas.toBlob(async (blob) => {
                if (!blob) {
                    console.error('Failed to convert canvas to blob');
                    return;
                }

                const arrayBuffer = await blob.arrayBuffer();
                const imageData = new Uint8Array(arrayBuffer);

                try {
                    const roundedImageData = await add_rounded_corners(imageData, canvas.width, canvas.height, radius);

                    if (roundedImageData.length === 0) {
                        console.error('Rounded image data is empty');
                        return;
                    }

                    const roundedBlob = new Blob([roundedImageData], { type: 'image/png' });
                    const roundedUrl = URL.createObjectURL(roundedBlob);
                    setRoundedImageSrc(roundedUrl);
                } catch (error) {
                    console.error('Error generating rounded image:', error);
                }
            }, 'image/png');
        };
        img.src = src;
    };

    const handleDownload = () => {
        if (roundedImageSrc) {
            const a = document.createElement('a');
            a.href = roundedImageSrc;
            a.download = 'rounded-image.png';
            a.click();
        }
    };

    return (
        <div>
            <input type="file" onChange={handleFileChange} accept="image/*" />
            {imageSrc && <img src={imageSrc} alt="Original" style={{ maxWidth: '128px', height: 'auto' }} />}
            <div>
                <label>
                    圆角半径（像素）:
                    <input type="number" value={radius} onChange={handleRadiusChange} />
                </label>
            </div>
            {roundedImageSrc && (
                <div>
                    <img src={roundedImageSrc} alt="Rounded" style={{ maxWidth: '128px', height: 'auto' }} />
                    <button onClick={handleDownload}>下载圆角图像</button>
                </div>
            )}
        </div>
    );
};

export default WasmRoundedIcon;
