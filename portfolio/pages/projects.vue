<script>
import VueJsonPretty from 'vue-json-pretty';
import 'vue-json-pretty/lib/styles.css';

export default {
    components: {
        VueJsonPretty
    }
}
</script>
<template>
    <Transition name="fade">
        <div class="page-inner">
            <div class="page-inner-1">
                <h6 class="text-h6 d-block mb-2" id="intro-name">
                    CASE STUDIES
                </h6>
                <h4 class="text-h3 d-block mb-4" id="intro-job">
                    Projects
                </h4>
                <TransitionGroup tag="div" name="slide">
                    <v-row>
                        <v-col cols="12" lg="3" class="">
                            <v-card class="mx-auto">
                                <v-img class="align-end text-white" height="200"
                                    src="https://cdn.vuetifyjs.com/images/cards/docks.jpg" cover>
                                    <v-card-title>Top 10 Australian beaches</v-card-title>
                                </v-img>

                                <v-card-subtitle class="pt-4">
                                    Number 10
                                </v-card-subtitle>

                                <v-card-text>
                                    <div>Whitehaven Beach</div>

                                    <div>Whitsunday Island, Whitsunday Islands</div>
                                </v-card-text>
                            </v-card>
                        </v-col>
                        <v-col cols="12" lg="3" class="">
                            <v-card class="mx-auto">
                                <v-img class="align-end text-white" height="200"
                                    src="https://cdn.vuetifyjs.com/images/cards/docks.jpg" cover>
                                    <v-card-title>Top 10 Australian beaches</v-card-title>
                                </v-img>

                                <v-card-subtitle class="pt-4">
                                    Number 10
                                </v-card-subtitle>

                                <v-card-text>
                                    <div>Whitehaven Beach</div>

                                    <div>Whitsunday Island, Whitsunday Islands</div>
                                </v-card-text>
                            </v-card>
                        </v-col>
                        <v-col cols="12" lg="3" class="">
                            <v-card class="mx-auto">
                                <v-img class="align-end text-white" height="200"
                                    src="https://cdn.vuetifyjs.com/images/cards/docks.jpg" cover>
                                    <v-card-title>Top 10 Australian beaches</v-card-title>
                                </v-img>

                                <v-card-subtitle class="pt-4">
                                    Number 10
                                </v-card-subtitle>

                                <v-card-text>
                                    <div>Whitehaven Beach</div>

                                    <div>Whitsunday Island, Whitsunday Islands</div>
                                </v-card-text>
                            </v-card>
                        </v-col>
                        <v-col cols="12" lg="3" class="">
                            <v-card class="mx-auto">
                                <v-img class="align-end text-white" height="200"
                                    src="https://cdn.vuetifyjs.com/images/cards/docks.jpg" cover>
                                    <v-card-title>Top 10 Australian beaches</v-card-title>
                                </v-img>

                                <v-card-subtitle class="pt-4">
                                    Number 10
                                </v-card-subtitle>

                                <v-card-text>
                                    <div>Whitehaven Beach</div>

                                    <div>Whitsunday Island, Whitsunday Islands</div>
                                </v-card-text>
                            </v-card>
                        </v-col>
                        <v-col cols="12" lg="3" class="">
                            <v-card class="mx-auto">
                                <v-img class="align-end text-white" height="200"
                                    src="https://cdn.vuetifyjs.com/images/cards/docks.jpg" cover>
                                    <v-card-title>Top 10 Australian beaches</v-card-title>
                                </v-img>

                                <v-card-subtitle class="pt-4">
                                    Number 10
                                </v-card-subtitle>

                                <v-card-text>
                                    <div>Whitehaven Beach</div>

                                    <div>Whitsunday Island, Whitsunday Islands</div>
                                </v-card-text>
                            </v-card>
                        </v-col>
                        <v-col cols="12" lg="3" class="">
                            <v-card class="mx-auto">
                                <v-img class="align-end text-white" height="200"
                                    src="https://cdn.vuetifyjs.com/images/cards/docks.jpg" cover>
                                    <v-card-title>Top 10 Australian beaches</v-card-title>
                                </v-img>

                                <v-card-subtitle class="pt-4">
                                    Number 10
                                </v-card-subtitle>

                                <v-card-text>
                                    <div>Whitehaven Beach</div>

                                    <div>Whitsunday Island, Whitsunday Islands</div>
                                </v-card-text>
                            </v-card>
                        </v-col>
                        <v-col cols="12" lg="3" class="">
                            <v-card class="mx-auto">
                                <v-img class="align-end text-white" height="200"
                                    src="https://cdn.vuetifyjs.com/images/cards/docks.jpg" cover>
                                    <v-card-title>Top 10 Australian beaches</v-card-title>
                                </v-img>

                                <v-card-subtitle class="pt-4">
                                    Number 10
                                </v-card-subtitle>

                                <v-card-text>
                                    <div>Whitehaven Beach</div>

                                    <div>Whitsunday Island, Whitsunday Islands</div>
                                </v-card-text>
                            </v-card>
                        </v-col>
                        <v-col cols="12" lg="3" class="">
                            <v-card class="mx-auto">
                                <v-img class="align-end text-white" height="200"
                                    src="https://cdn.vuetifyjs.com/images/cards/docks.jpg" cover>
                                    <v-card-title>Top 10 Australian beaches</v-card-title>
                                </v-img>

                                <v-card-subtitle class="pt-4">
                                    Number 10
                                </v-card-subtitle>

                                <v-card-text>
                                    <div>Whitehaven Beach</div>

                                    <div>Whitsunday Island, Whitsunday Islands</div>
                                </v-card-text>
                            </v-card>
                        </v-col>

                    </v-row>
                </TransitionGroup>
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
    display: block;
}
</style>