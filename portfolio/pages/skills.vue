<!-- https://ismail9k.github.io/vue3-carousel/examples.html -->
<!-- https://www.abusaid.me/#experience -->
<script>
import VueSlickCarousel from "vue-slick-carousel";
import "vue-slick-carousel/dist/vue-slick-carousel.css";
// optional style for arrows & dots
import "vue-slick-carousel/dist/vue-slick-carousel-theme.css";

export default {
  components: {
    VueSlickCarousel,
  },
};
</script>
<template>
  <div class="page-inner">
    <div class="page-inner-1">
      <VueSlickCarousel :arrows="true" :dots="true">
        <div>1</div>
        <div>2</div>
        <div>3</div>
        <div>4</div>
      </VueSlickCarousel>
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
  display: block;
  display: flex;
  align-items: center;
}
</style>