<script lang="ts">
import { useNotifier } from "vuetify-notifier";
import { object, string } from "yup";
import * as Yup from 'yup';

const contactFormSchema = object().shape({
    name: string().required(),
    email: string().required().email(),
    message: string().required()
});

export default {
    setup() {
        const $notifier = useNotifier();
        return { $notifier: $notifier }
    },
    components: {

    },
    data() {
        return {
            valid: false,
            item: {
                name: '',
                email: '',
                message: ''

            },
            loading: false
        }

    },
    methods: {
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
        async saveContact() {
            this.valid = await contactFormSchema.isValid(this.item);
            if (this.valid) {
                this.loading = true
                const res = await $fetch('https://api-portfolio.pearl2201.com/api/mails', {
                    method: 'POST',
                    body: this.item
                });
                this.loading = false
            }
        },
    }
}
</script>
<template>
    <Transition name="fade" :duration="3000" appear>
        <div class="page-inner">
            <div class="page-inner-1">

                <h6 class="text-h6 d-block mb-2" id="intro-name">
                    Get in touch
                </h6>
                <h4 class="text-h3 d-block mb-4" id="intro-job">
                    Contact
                </h4>
                <v-row>
                    <v-col cols="12" lg="6">

                        <v-form ref="form" v-model="valid" @submit.prevent="saveContact">
                            <v-text-field v-model="item.name" :rules="[() => validateCreatedItem('name')]"
                                label="Your name"></v-text-field>
                            <v-text-field v-model="item.email" :rules="[() => validateCreatedItem('email')]"
                                label="Your email"></v-text-field>
                            <v-textarea v-model="item.message" :rules="[() => validateCreatedItem('message')]"
                                label="Your message"></v-textarea>
                            <v-btn color="primary" class="mt-2" type="submit" block> <v-progress-circular :size="20"
                                    :width="1" color="green" indeterminate class="mr-2" v-if="loading"></v-progress-circular> Send
                                <v-icon class="ml-2">
                                    mdi-mail</v-icon></v-btn>
                        </v-form>


                    </v-col>

                    <v-col cols="12" lg="6" class="d-flex flex-column justify-center">
                        <div class="mx-auto">
                            <v-row>
                                <v-col class="d-flex align-end">
                                    <v-icon variant="text" size="x-large" class="mr-2">mdi-city</v-icon>
                                    <p>Hanoi, Viet Nam</p>
                                </v-col>
                            </v-row>
                            <v-row>
                                <v-col class="d-flex">
                                    <v-btn icon="mdi-github" variant="text" href="https://github.com/pearl2201"
                                        target="_blank"></v-btn>
                                    <v-btn icon="mdi-linkedin" variant="text"
                                        href="https://vn.linkedin.com/in/nguy%E1%BB%85n-anh-ng%E1%BB%8Dc-a3a94a36"
                                        target="_blank"></v-btn>
                                    <v-btn icon="mdi-facebook" variant="text" href="https://www.facebook.com/pearl.2201"
                                        target="_blank"></v-btn>
                                    <v-btn icon="mdi-twitter" variant="text" href="https://twitter.com/pearl2201"
                                        target="_blank"></v-btn>
                                </v-col>
                            </v-row>
                        </div>

                    </v-col>
                </v-row>

            </div>
        </div>
    </Transition>
</template>

<style scoped>
.vjs-key {
    color: cornflowerblue
}

#intro-name {
    color: cornflowerblue;
}

#intro-job {
    color: coral
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
    display: flex;
    flex-direction: column;
}
</style>