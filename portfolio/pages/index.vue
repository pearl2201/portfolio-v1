<script>
import VueJsonPretty from 'vue-json-pretty';

export default {
    components: {
        VueJsonPretty
    }
}
</script>
<template>
    <div class="page-inner" >
       <div class="page-inner-1" >
            <v-row>
                <v-col cols="12" lg="6" class="">
                    <v-row>
                        <v-col class="text-h4">
                            Hello,<br />
                            This is <span id="intro-name">Anh Ngọc</span>, I'm a<br />
                            Profressional <span id="intro-job">Software<br />
                                Developer</span>.
                        </v-col>
                    </v-row>
                    <v-row>
                        <v-col class="d-flex">
                            <v-btn icon="mdi-github" variant="text"></v-btn>
                            <v-btn icon="mdi-linkedin" variant="text"></v-btn>
                            <v-btn icon="mdi-facebook" variant="text"></v-btn>
                            <v-btn icon="mdi-twitter" variant="text"></v-btn>

                        </v-col>
                    </v-row>
                    <v-row>
                        <v-col>
                            <v-btn class="mr-2" color="primary"> CONTACT ME <v-icon icon="mdi-account-box"></v-icon></v-btn>

                            <v-btn color="secondary"> GET RESUME <v-icon icon="mdi-download"></v-icon></v-btn>
                        </v-col>
                    </v-row>
                </v-col>
                <v-col cols="12" lg="6">
                    <v-row id="window-top">
                        <v-col>
                            <v-icon icon="mdi-circle" variant="text" color="red" size="x-small" class="mr-1"></v-icon>
                            <v-icon icon="mdi-circle" variant="text" color="green" size="x-small" class="mr-1"></v-icon>
                            <v-icon icon="mdi-circle" variant="text" color="blue" size="x-small"></v-icon>
                        </v-col>
                    </v-row>
                    <v-row id="window-body">
                        <v-col>
                            <vue-json-pretty
                                :data="{ name: 'Nguyễn Anh Ngọc', skills: ['Vue', 'C#', 'Python', 'PostgreSQL', 'Docker', 'Kubernetes', 'AWS'], hardWorker: true, quickLearner: true, problemSolver: true, hireable: true }" />
                        </v-col>
                    </v-row>
                </v-col>
            </v-row>
        </div>
    </div>

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

.page-inner-1{
    width:100%;
    height:auto;
    display:block;
    display: flex;
    align-items: center;
}
</style>