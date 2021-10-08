<template>
    <div class="container">
        <div class="row my-2">
            <input type="file" ref="file" @change="loadImage">
        </div>
        <div class="row my-2">
            <div class="col ">
                <button class="btn btn-primary" type="button" @click="original">オリジナル</button>
            </div>
            <div class="col">
                <button class="btn btn-primary" type="button" @click="grayscale">グレースケール</button>
            </div>
            <div class="col">
                <button class="btn btn-primary" type="button" @click="flip_horizontal">左右反転</button>
            </div>
            <div class="col">
                <button class="btn btn-primary" type="button" @click="flip_vertical">上下反転</button>
            </div>
            <div class="col">
                <button class="btn btn-primary" type="button" @click="rotate_left">左回転</button>
            </div>
            <div class="col">
                <button class="btn btn-primary" type="button" @click="rotate_right">右回転</button>
            </div>
            <div class="col">
                <button class="btn btn-primary" type="button" @click="blur">ぼかし</button>
            </div>
        </div>
        <div class="row my-2">
            <div class="col">
                <h4>Original</h4>
                <img ref="original" :style="original_style">
            </div>
            <div class="col">
                <h4>Converted</h4>
                <canvas ref="converted" :style="style"></canvas>
            </div>
        </div>
    </div>
</template>

<script>

const ConvertMode = {
    Grayscale: 1,
    FlipHorizontal: 2,
    FlipVertical: 3,
    RotateLeft: 4,
    RotateRight: 5,
    Blur: 6,
};

const MAX_IMAGE_SIZE = 500;

export default {
    name: 'App',
    data () {
        return {
            wasm: null,
            original_width: 0,
            original_height: 0,
            width: 0,
            height: 0,
        };
    },
    created () {
        this.loadWasm();
    },
    computed: {
        style () {
            return {
                width: this.width + "px",
                height: this.height + "px",
            };
        },
        original_style () {
            return {
                width: this.original_width + "px",
                height: this.original_height + "px",
            };
        },
    },
    methods: {
        async loadWasm() {
            const wasm = import("../wasm/pkg");

            this.wasm = await wasm;
        },
        readFile () {
            const input = this.$refs.file;
            if (!input.files.length) return;

            const file = input.files[0];

            const reader = new FileReader();
            const p = new Promise((resolve, reject) => {
                reader.addEventListener("load", function () {
                    resolve(reader.result);
                });
                reader.addEventListener("error", function () {
                    reject("error.");
                });
            });

            reader.readAsDataURL(file);

            return p;
        },
        drawOriginalImage(url) {
            const img = this.$refs.original;

            const p = new Promise((resolve) => {
                img.addEventListener("load", function () {
                    resolve();
                });
            });

            img.src = url;

            if (img.complete) {
                return Promise.resolve();
            }

            return p;
        },
        original () {
            const out = this.$refs.converted;

            const out_ctx = out.getContext('2d');

            const img = this.$refs.original;

            this.width = this.original_width;
            this.height = this.original_height;

            const max = Math.max(this.width, this.height);
            out_ctx.clearRect(0, 0, max, max);
            out_ctx.drawImage(img, 0, 0, this.original_width, this.original_height);
        },
        convert (mode) {
            const canvas = this.$refs.converted;

            const ctx = canvas.getContext('2d');

            const raw = ctx.getImageData(0, 0, this.width, this.height);

            let ret_data;
            switch (mode) {
                case ConvertMode.Grayscale:
                    ret_data = this.wasm.grayscale(this.width, this.height, raw.data);
                    break;
                case ConvertMode.FlipHorizontal:
                    ret_data = this.wasm.flip_horizontal(this.width, this.height, raw.data);
                    break;
                case ConvertMode.FlipVertical:
                    ret_data = this.wasm.flip_vertical(this.width, this.height, raw.data);
                    break;
                case ConvertMode.RotateLeft:
                    ret_data = this.wasm.rotate_left(this.width, this.height, raw.data);
                    break;
                case ConvertMode.RotateRight:
                    ret_data = this.wasm.rotate_right(this.width, this.height, raw.data);
                    break;
                case ConvertMode.Blur:
                    ret_data = this.wasm.blur(this.width, this.height, raw.data);
                    break;
            }

            const buf = Uint8ClampedArray.from(ret_data);

            if (mode === 4 || mode === 5) {
                const w = this.width;
                const h = this.height;
                this.width = h;
                this.height = w;
                canvas.width = this.width;
                canvas.height = this.height;
            }

            const max = Math.max(this.width, this.height);
            ctx.clearRect(0, 0, max, max);
            ctx.putImageData(new ImageData(buf, this.width, this.height), 0, 0);
        },
        grayscale () {
            this.convert(ConvertMode.Grayscale)
        },
        flip_horizontal () {
            this.convert(ConvertMode.FlipHorizontal);
        },
        flip_vertical () {
            this.convert(ConvertMode.FlipVertical);
        },
        rotate_left() {
            this.convert(ConvertMode.RotateLeft);
        },
        rotate_right() {
            this.convert(ConvertMode.RotateRight);
        },
        blur() {
            this.convert(ConvertMode.Blur);
        },
        async loadImage () {
            const url = await this.readFile();
            await this.drawOriginalImage(url);

            const img = this.$refs.original;

            const w = Math.min(img.naturalWidth, MAX_IMAGE_SIZE);
            const h = img.naturalHeight * (w / img.naturalWidth);

            //this.width = this.original_width = img.naturalWidth;
            //this.height = this.original_height = img.naturalHeight;
            this.width = this.original_width = w;
            this.height = this.original_height = h

            const out = this.$refs.converted;
            out.width = this.width;
            out.height = this.height;

            this.original();
        },
    },
};
</script>

<style lang="scss">
#app {
    img, canvas {
        margin: 0;
        padding: 0;
    }
}
</style>
