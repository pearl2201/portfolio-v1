
<template>
    <v-app>
        <v-main>
            <v-container class="page-wrapper pb-sm-15 pb-10">
                <div class='maxWidth d-flex flex-column'>
                    <div class="d-flex mb-4">
                        <v-btn variant="plain" flat class="text-h4" to="/">Pearl2201</v-btn>
                        <v-list class="ml-auto d-flex">
                            <v-list-item to="/">ABOUT</v-list-item>
                            <v-list-item to="/experience">EXPERIENCE</v-list-item>
                            <v-list-item to="/skills">SKILLS</v-list-item>
                            <v-list-item to="/tools">TOOLS</v-list-item>
                            <!-- <v-list-item to="/projects">PROJECTS</v-list-item> -->
                            <v-list-item to="/contact">CONTACT</v-list-item>
                        </v-list>
                    </div>



                </div>

                <NuxtPage />


            </v-container>

        </v-main>
        <v-footer class="bg-indigo-lighten-1 text-center d-flex flex-column justify-center" :height="100">
            <div>
                {{ new Date().getFullYear() }} — <strong>Pearl2201</strong><br/>
                <v-btn variant="text" href="https://github.com/pearl2201/portfolio-v1"
                                        target="_blank"> <v-icon icon="mdi-github" class="mr-2"></v-icon>Source code this website</v-btn>
            </div>
        </v-footer>
    </v-app>
</template>

<style>
.page-wrapper {
    min-height: calc(100vh - 100px);
    display: flex;
    flex-direction: column;
}


</style>