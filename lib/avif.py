from PIL import Image
import pillow_avif
import sys
import io
import base64

def image_convert(path):
    image = Image.open(path)
    image_stream = io.BytesIO()
    image.save(image_stream, format="JPEG")
    image_stream.seek(0)
    image_base64 = base64.b64encode(image_stream.getvalue()).decode('utf-8')
    return image_base64

if __name__ == "__main__":
    if len(sys.argv) > 1:
        path = sys.argv[1]
        base64_image = image_convert(path)
        print(base64_image,end='')
    else:
        print("Usage: python script.py <path_to_image>")