// @ts-check
/// <reference path="./global.d.ts" />

/** @type { (size: number) => Promise<any> } */
const initWebcam = async (size) => {
    const flip = true;
    const webcam = new tmPose.Webcam(size, size, flip);
    await webcam.setup();
    await webcam.play();
    return webcam;
}

/** @type { (size: number) => CanvasRenderingContext2D } */
const initCanvas = (size) => {
    const canvas = /** @type {HTMLCanvasElement} */ (document.getElementById("camera"));
    canvas.width = canvas.height = size;
    return /** @type {CanvasRenderingContext2D} */ (canvas.getContext("2d"));
}

/**
 * @param url {string}
*/
const loadModel = async (url) => {
    const modelURL = `${url}/model.json`;
    const metadataURL = `${url}/metadata.json`;
    return await tmPose.load(modelURL, metadataURL);
}

class Camera {
    #webcam;
    /** @type {CanvasRenderingContext2D} */
    #ctx;

    /**
     * @param {any} webcam
     * @param {CanvasRenderingContext2D} ctx
     */
    constructor(webcam, ctx) {
        this.#webcam = webcam;
        this.#ctx = ctx;

        const loop = () => {
            this.#webcam.update();
            requestAnimationFrame(loop);
        }
        requestAnimationFrame(loop);
    }

    /**
     * @param {{ keypoints: any; }} pose
     */
    #drawPose(pose) {
        this.#ctx.drawImage(this.#webcam.canvas, 0, 0);
        if (!pose) {
            return;
        }
        const minPartConfidence = 0.5;
        tmPose.drawKeypoints(pose.keypoints, minPartConfidence, this.#ctx);
        tmPose.drawSkeleton(pose.keypoints, minPartConfidence, this.#ctx);
    }

    async estimatePose(model) {
        const { pose, posenetOutput } = await model.estimatePose(this.#webcam.canvas);
        this.#drawPose(pose);
        return posenetOutput;
    }
}

/**
* @param camera {Camera}
* @param model any
*/
const predict = async (camera, model) => {
    const posenetOutput = await camera.estimatePose(model);
    const maxPredictions = 1;
    const prediction = await model.predictTopK(posenetOutput, maxPredictions);
    return prediction[0].className;
}

class PredictionPolling {
    #result = undefined;
    /** @type undefined | Promise<undefined> */
    #promise = undefined;

    /** @type {Camera} */
    #camera;
    #model;


    /**
     * @param {Camera} camera
     * @param {any} model
     */
    constructor(camera, model) {
        this.#camera = camera;
        this.#model = model;
    }

    pollOrInvoke() {
        if (this.#result !== undefined) {
            const result = this.#result;
            this.#result = undefined;
            return result;
        }

        if (this.#promise === undefined) {
            this.#promise = predict(this.#camera, this.#model)
                .then((className) => this.#result = className)
                .catch(() => console.error("Prediction failed."))
                .then(() => this.#promise = undefined);
        }
        return "None";
    }
}

const init = async () => {
    const size = 250;
    const ctx = initCanvas(size);
    const [webcam, movementModel, rotationModel] = await Promise.all(
        [initWebcam(size), loadModel("models/movement"), loadModel("models/rotation")]
    );
    const camera = new Camera(webcam, ctx);

    const movementPrediction = new PredictionPolling(camera, movementModel);
    const rotationPrediction = new PredictionPolling(camera, rotationModel);

    return { movementPrediction, rotationPrediction };
}

const { movementPrediction, rotationPrediction } = await init();

export const getMovementPrediction = () => {
    return movementPrediction.pollOrInvoke();
}

export const getRotationPrediction = () => {
    return rotationPrediction.pollOrInvoke();
}
