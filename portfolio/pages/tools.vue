<!-- https://ismail9k.github.io/vue3-carousel/examples.html -->
<!-- https://www.abusaid.me/#experience -->
<!-- https://shaq-portfolio.netlify.app/#contact -->
<script lang="ts">
import { useNotifier } from "vuetify-notifier";
import { object, string, number } from "yup";
import * as Yup from 'yup';
import axios from "axios";

const contactFormSchema = object().shape({
    size: number().required().min(100).max(400),
    darkColor: string().required().matches(/^#(?:[0-9a-fA-F]{3,4}){1,2}$/),
    lightColor: string().required().matches(/^#(?:[0-9a-fA-F]{3,4}){1,2}$/),
    content: string().required()
});

export default {
    components: {


    },
    data() {
        return {
            valid: false,
            loading: false,
            item: {
                content: '',
                size: 200,
                darkColor: "#000",
                lightColor: "#fff"
            },
            qrcodeSvg: ''
        }
    },
    methods: {
        async generateQrCode() {
            this.valid = await contactFormSchema.isValid(this.item);
            if (this.valid) {
                this.loading = true
                try {
                    const res = await axios.post('https://worker-rust.nguyenanhngoc-ftu9932.workers.dev/generate', {
                        content: this.item.content,
                        size: Number.parseInt(this.item.size),
                        dark_color: this.item.darkColor,
                        light_color: this.item.lightColor

                    });
                    this.qrcodeSvg = (res as any).data.message
                }
                catch (err) {
                    console.error(err)
                } finally {
                    this.loading = false
                }
            }
        },
        async validateCreatedItem(field: string) {
            try {
                await contactFormSchema
                    .validateAt(field, this.item)
                return true;
            }
            catch (err: any) {
                return err.message;
            }
        },
    }
};
</script>
<template>


    <div class="page-inner">
        <div class="page-inner-1">
            <h6 class="text-h6 d-block mb-2" id="intro-name">
                MY TOOLS
            </h6>
            <h4 class="text-h3 d-block mb-4" id="intro-job">
                Some of my useful tools
            </h4>
            <v-divider />
            <div class="mt-4">
                <v-row>
                    <v-col cols="8">
                        <h4 class="text-h6 d-block mb-4" id="intro-tools">
                            QrCode generator
                        </h4>
                        <v-form ref="form" v-model="valid" @submit.prevent="generateQrCode">
                            <v-row>
                                <v-col>
                                    <v-text-field v-model="item.content" :rules="[() => validateCreatedItem('content')]"
                                        label="Content" placeholder="Your qr code content"></v-text-field>
                                </v-col>
                                <v-col>
                                    <v-text-field v-model="item.size" :rules="[() => validateCreatedItem('size')]"
                                        label="Size" type="number" min="100" max="400"></v-text-field>
                                </v-col>
                            </v-row>
                            <v-row>
                                <v-col>
                                    <h6 class="text-button">Dark Color</h6>
                                    <v-color-picker v-model="item.darkColor"
                                        :rules="[() => validateCreatedItem('darkColor')]"
                                        label="Dark color"></v-color-picker>
                                </v-col>
                                <v-col>
                                    <h6 class="text-button">Light Color</h6>
                                    <v-color-picker v-model="item.lightColor"
                                        :rules="[() => validateCreatedItem('lightColor')]"
                                        label="Light color"></v-color-picker>
                                </v-col>
                            </v-row>
                            <v-btn color="primary" class="mt-2" type="submit"> <v-progress-circular :size="20"
                                    :width="1" color="green" indeterminate class="mr-2"
                                    v-if="loading"></v-progress-circular> Generate</v-btn>
                        </v-form>
                    </v-col>
                    <v-col cols="4" class="d-flex align-center justify-center">
                        <div v-if="qrcodeSvg" v-html="qrcodeSvg"></div>
                    </v-col>
                </v-row>
                <v-row>
                    <v-col cols="8">
                        <h4 class="text-h6 d-block mb-4" id="intro-tools">
                            Simple chat app
                        </h4>
                        <p>Simple chat app made with Elixir Phoenix and VueJs. You could try it here <a href="https://chat.pearl2201.com/">https://chat.pearl2201.com/</a></p>
                      </v-col>
                </v-row>
            </div>
        </div>
    </div>
</template>

<style>
.vjs-key {
    color: cornflowerblue;
}

#intro-name {
    color: cornflowerblue;
}

#intro-job {
    color: coral;
}

#window-top {
    border: 1px solid black;
}

#window-body {
    border: 1px solid black;
}

.page-inner {
    height: 100%;
    flex-grow: 1;
    display: flex;
    align-items: center;
}

.page-inner-1 {
    width: 100%;
    height: auto;
}

.logo-skill {
    width: 120px;
    height: 120px;
}
</style>
