const MJPEG_URL = 'http://192.168.1.9:81/stream'; // replace with your stream

const canvas = document.getElementById('main-canvas');
const ctx = canvas.getContext('2d');

const image = new Image();
image.crossOrigin = "anonymous"; // optional, if needed

let buffer = new Uint8Array(0);

// Decode each JPEG frame and draw to canvas
function handleJPEGFrame(jpegBlob) {
  const imgURL = URL.createObjectURL(jpegBlob);
  image.onload = () => {
    ctx.drawImage(image, 0, 0, canvas.width, canvas.height);
    // const pixelData = ctx.getImageData(0, 0, canvas.width, canvas.height).data;
    console.log("here")
    // console.log("Got pixel data!", pixelData);
    URL.revokeObjectURL(imgURL);
  };
  image.src = imgURL;
}

// Connect to the MJPEG stream
function startMJPEGStream(url) {
  fetch(url).then(response => {
    const reader = response.body.getReader();
    const boundary = '--' + /boundary=(.*)/i.exec(response.headers.get("Content-Type"))[1];

    let currentJPEG = [];
    let readingJPEG = false;

    function readChunk() {
      reader.read().then(({ done, value }) => {
        if (done) return;

        const chunkStr = new TextDecoder("latin1").decode(value);
        const boundaryIndex = chunkStr.indexOf(boundary);

        if (boundaryIndex !== -1) {
          const jpegStart = chunkStr.indexOf('\r\n\r\n', boundaryIndex) + 4;
          const rawJPEG = value.slice(jpegStart);

          // Close previous blob
          if (readingJPEG && currentJPEG.length > 0) {
            const blob = new Blob(currentJPEG, { type: 'image/jpeg' });
            handleJPEGFrame(blob);
            currentJPEG = [];
          }

          readingJPEG = true;
          currentJPEG.push(rawJPEG);
        } else if (readingJPEG) {
          currentJPEG.push(value);
        }

        readChunk();
      });
    }

    readChunk();
  });
}

startMJPEGStream(MJPEG_URL);