<script>
import VueJsonPretty from "vue-json-pretty";
import "vue-json-pretty/lib/styles.css";

export default {
  components: {
    VueJsonPretty,
  },
};
</script>
<template>
  <div class="page-inner">
    <div class="page-inner-1">
      <h6 class="text-h6 mb-2" id="intro-name">
        WHAT I'VE DONE SO FAR
      </h6>
      <h4 class="text-h3 mb-4" id="intro-job">
        Work Experience
      </h4>
      <v-timeline direction="horizontal">
        <v-timeline-item>
          <template v-slot:opposite> 04/2014 - 12/2015 </template>
          <div>
            <div class="text-h6">Tofu Game Studio</div>
            <p class="text-subtitle-1">
              Game Developer
            </p>
            <p class="text-caption">
              Developer mobile game with libgdx library and Unity3d Engine
            </p>
          </div>
        </v-timeline-item>

        <v-timeline-item>
          <template v-slot:opposite> 01/2016 - 01/2020 </template>
          <div>
            <div class="text-h6">Kyx Studio</div>
            <p class="text-subtitle-1">
              Game Developer
            </p>
            <p class="text-caption">
              Developer mobile game with Unity3d Engine
            </p>
          </div>
        </v-timeline-item>

        <v-timeline-item>
          <template v-slot:opposite> 01/2020 - 03/2023 </template>
          <div>
            <div class="text-h6">Crosgames Studio</div>
            <p class="text-subtitle-1">
              Backend Developer
            </p>
            <p class="text-caption">
              Design and develop backend for games and application. Manage Aws infrastructure and gitlab ci-ci. Support
              Unity
              team in third party integration.
            </p>
          </div>
        </v-timeline-item>

        <v-timeline-item>
          <template v-slot:opposite> 04/2023 - present </template>
          <div>
            <div class="text-h6">Inwave Studio</div>
            <p class="text-subtitle-1">
              Backend Developer
            </p>
            <p class="text-caption">
              Design and develop backend for games and application. Manage Aws infrastructure and gitlab ci-ci.
            </p>
          </div>
        </v-timeline-item>
      </v-timeline>
    </div>
  </div>
</template>

<style scoped>
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
  display: block;
}
</style>