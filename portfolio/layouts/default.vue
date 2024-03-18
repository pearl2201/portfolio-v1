<template>
    <v-app>
        <v-main>
            <v-container fluid class="page-wrapper pb-sm-15 pb-10">
                <div class='maxWidth'>
                    <div class="d-flex">
                        <div class="text-h4">Pearl2201</div>
                        <v-list class="ml-auto d-flex">
                            <v-list-item>ABOUT</v-list-item>
                            <v-list-item>EXPERIENCE</v-list-item>
                            <v-list-item>SKILLS</v-list-item>
                            <v-list-item>EDUCATION</v-list-item>
                            <v-list-item>PROJECTS</v-list-item>
                        </v-list>
                    </div>
                    <NuxtPage />

                </div>
            </v-container>

        </v-main>
        <v-footer class="bg-indigo-lighten-1 text-center d-flex flex-column" :height="100">
            <- <div>
                {{ new Date().getFullYear() }} — <strong>Pearl2201</strong>
                </div>
        </v-footer>
    </v-app>
</template>

<style scoped>
.page-wrapper {
    min-height: calc(100vh - 100px);
}
</style>