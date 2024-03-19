<template>
    <v-app>
        <v-main>
            <v-container class="page-wrapper pb-sm-15 pb-10">
                <div class='maxWidth d-flex flex-column'>
                    <div class="d-flex mb-4">
                        <div class="text-h4">Pearl2201</div>
                        <v-list class="ml-auto d-flex">
                            <v-list-item to="/">ABOUT</v-list-item>
                            <v-list-item to="/experience">EXPERIENCE</v-list-item>
                            <v-list-item to="/skills">SKILLS</v-list-item>
                            <v-list-item to="/education">EDUCATION</v-list-item>
                            <v-list-item to="/projects">PROJECTS</v-list-item>
                        </v-list>
                    </div>


                </div>

                <NuxtPage />


            </v-container>

        </v-main>
        <v-footer class="bg-indigo-lighten-1 text-center d-flex flex-column" :height="100">
            <div>
                {{ new Date().getFullYear() }} — <strong>Pearl2201</strong>
            </div>
        </v-footer>
    </v-app>
</template>

<style scoped>
.page-wrapper {
    min-height: calc(100vh - 100px);
    display: flex;
    flex-direction: column;
}
</style>