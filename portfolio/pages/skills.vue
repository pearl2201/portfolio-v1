<!-- https://ismail9k.github.io/vue3-carousel/examples.html -->
<!-- https://www.abusaid.me/#experience -->
<!-- https://shaq-portfolio.netlify.app/#contact -->
<script>
import { VueMarqueeSlider } from 'vue3-marquee-slider';
import '@/node_modules/vue3-marquee-slider/dist/style.css'

export default {
  components: {
    VueMarqueeSlider,
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
        <vue-marquee-slider id="marquee-slider1" :speed="4040" :width="120">
          <div class="logo-skill">
            <img src="https://app.imgforce.com/images/user/zrC_1622176244_logo-black-120.png" width="120"
              height="120" />
          </div>
          <div class="logo-skill">
            <img src="https://app.imgforce.com/images/user/O1j_1670884991_js-logo.png" width="120" height="120" />
          </div>
          <div class="logo-skill">
            <img src="https://app.imgforce.com/images/user/Igx_1670885749_vue-logo.png" width="120" height="120" />
          </div>
          <div class="logo-skill">
            <img src="https://app.imgforce.com/images/user/TPs_1670885858_react-logo.png" width="120" height="120" />
          </div>
          <div class="logo-skill">
            <img src="https://app.imgforce.com/images/user/jY4_1670885309_angular-logo.png" width="120" height="120" />
          </div>
        </vue-marquee-slider>
      </div>
      <div>
        <vue-marquee-slider id="marquee-slider2" :speed="4040" :width="120">
          <div class="logo-skill">
            <img src="https://app.imgforce.com/images/user/zrC_1622176244_logo-black-120.png" width="120"
              height="120" />
          </div>
          <div class="logo-skill">
            <img src="https://app.imgforce.com/images/user/O1j_1670884991_js-logo.png" width="120" height="120" />
          </div>
          <div class="logo-skill">
            <img src="https://app.imgforce.com/images/user/Igx_1670885749_vue-logo.png" width="120" height="120" />
          </div>
          <div class="logo-skill">
            <img src="https://app.imgforce.com/images/user/TPs_1670885858_react-logo.png" width="120" height="120" />
          </div>
          <div class="logo-skill">
            <img src="https://app.imgforce.com/images/user/jY4_1670885309_angular-logo.png" width="120" height="120" />
          </div>
        </vue-marquee-slider>
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