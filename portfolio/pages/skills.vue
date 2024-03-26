<!-- https://ismail9k.github.io/vue3-carousel/examples.html -->
<!-- https://www.abusaid.me/#experience -->
<!-- https://shaq-portfolio.netlify.app/#contact -->
<script>
import IconReact from '@/assets/svg/skills/react.svg'
import IconVue from '@/assets/svg/skills/vue.svg'
import IconTypescript from '@/assets/svg/skills/typescript.svg'
import IconVuetify from '@/assets/svg/skills/vuetifyjs.svg'
import IconUnity from '@/assets/svg/skills/unity.svg'
import IconNuxt from '@/assets/svg/skills/nuxtJS.svg'
import IconCsharp from '@/assets/svg/skills/csharp.svg'
import IconAws from '@/assets/svg/skills/aws.svg'
import IconDocker from '@/assets/svg/skills/docker.svg'
import IconPython from '@/assets/svg/skills/python.svg'
import IconPostgresql from '@/assets/svg/skills/postgresql.svg'
export default {
  components: {
    IconReact,
    IconVue,
    IconTypescript,
    IconVuetify,
    IconUnity,
    IconNuxt,
    IconCsharp,
    IconAws,
    IconDocker,
    IconPython,
    IconPostgresql

  },
};
</script>
<template>


  <div class="page-inner">
    <div class="page-inner-1">
      <h6 class="text-h6 d-block mb-2" id="intro-name">
        MY SKILLS
      </h6>
      <h4 class="text-h3 d-block mb-4" id="intro-job">
        Technologies
      </h4>
      <div>

        <TransitionGroup tag="div" name="slide" class="mb-8 d-flex justify-space-between align-center items-center">
          <div class="logo-skill">
            <IconTypescript class="text-xl" :fontControlled="false" filled />
          </div>
          <div class="logo-skill">
            <IconReact class="text-xl" :fontControlled="false" filled />
          </div>
          <div class="logo-skill">
            <IconVue class="text-xl" :fontControlled="false" filled />
          </div>
          <div class="logo-skill">
            <IconNuxt class="text-xl" :fontControlled="false" filled />
          </div>
          <div class="logo-skill">
            <IconUnity class="text-xl" :fontControlled="false" filled />
          </div>
        </TransitionGroup>

      </div>
      <div>
        <div id="marquee-slider2" :speed="4040" :width="120"
          class="d-flex d-flex justify-space-between align-center items-center">
          <div class="logo-skill">
            <IconCsharp class="text-xl" :fontControlled="false" filled />
          </div>
          <div class="logo-skill">
            <IconPython class="text-xl" :fontControlled="false" filled />
          </div>
          <div class="logo-skill">
            <IconPostgresql class="text-xl" :fontControlled="false" filled />
          </div>
          <div class="logo-skill">
            <IconDocker class="text-xl" :fontControlled="false" filled />
          </div>
          <div class="logo-skill">
            <IconAws class="text-xl" :fontControlled="false" filled />
          </div>

        </div>
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