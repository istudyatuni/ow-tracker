<svelte:options namespace="svg" />

<script module>
  import { getContext, onMount } from "svelte";
  import * as L from "leaflet";

  const SCALE = 0.95;

  const TEXT_HEIGHT = 100 * SCALE;
  const FONT_SIZE_EM = 1.8 * SCALE;

  const IMAGE_WIDTH = 200 * SCALE;
  const IMAGE_HEIGHT = IMAGE_WIDTH;

  const CARD_HEIGHT = IMAGE_HEIGHT + TEXT_HEIGHT;
  const CARD_WIDTH = IMAGE_WIDTH;
  const CARD_MARGIN = 2 * SCALE;

  const FULL_CARD_WIDTH = CARD_WIDTH + CARD_MARGIN * 2;
  const FULL_CARD_HEIGHT = CARD_HEIGHT + CARD_MARGIN;

  const STAR_PATH =
    "M3 12a9 9 0 1 0 18 0a9 9 0 0 0-18 0m9-4v8m3.5-6l-7 4m7 0l-7-4";
  const STAR_SIZE = 50;

  const QUESTION = {
    x: IMAGE_WIDTH / 3,
    y: TEXT_HEIGHT + IMAGE_HEIGHT / 1.4,
    font: (IMAGE_HEIGHT * 2) / 3,
  };
</script>

<script>
  let {
    // Unique id, used for detecting clicked element.
    id,
    text,
    bounds,
    pane,
    image_url = null,
    has_unexplored = false,
    category_class,
  } = $props();

  // increase card width for star
  // increase on both sides to not shift card on map (because of actual card's center change)
  let has_star = $derived(image_url !== null && has_unexplored);
  let left_shift = $derived(has_star ? STAR_SIZE : 0);
  let additional_width = $derived(has_star ? STAR_SIZE * 2 : 0);

  let innerRectSizes = $derived({
    x: CARD_MARGIN + left_shift,
    y: TEXT_HEIGHT,
    width: IMAGE_WIDTH,
    height: IMAGE_HEIGHT,
  });

  let self = null;

  onMount(() =>
    L.svgOverlay(self, bounds, { pane }).addTo(getContext("map")()),
  );
</script>

<svg
  viewBox="0 0 {FULL_CARD_WIDTH + additional_width} {FULL_CARD_HEIGHT}"
  bind:this={self}
  class={category_class}>
  <rect
    x={left_shift}
    y="0"
    {id}
    width={FULL_CARD_WIDTH}
    height={FULL_CARD_HEIGHT}
    class="card {category_class}" />
  <switch>
    <!-- foreignObject is used to use <p> to have text auto-wrap -->
    <foreignObject x={left_shift} y="0" width={CARD_WIDTH} height={TEXT_HEIGHT}>
      <div class="card-text-wrapper">
        <p class="card-text mono spoiler" style="font-size: {FONT_SIZE_EM}em">
          {@html text}
        </p>
      </div>
    </foreignObject>
    <text x={left_shift} y="0" font-size="20" text-anchor="middle" fill="white"
      >svg viewer doesn't support html</text>
  </switch>

  {#if image_url !== null}
    <image href={image_url} {...innerRectSizes} class="spoiler" />

    {#if has_unexplored}
      <!-- i need to customize <path> -->
      <!-- todo: probably it's possible to get it from "~icons" -->
      <!-- tabler:medical-cross-circle -->
      <!-- <CrossIcon class="explore-star" /> -->
      <path
        fill="none"
        class="explore-star"
        transform="translate(250, 0) scale(2, 2)"
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d={STAR_PATH} />
    {/if}
  {:else}
    <rect {...innerRectSizes} class="img-q-bg" />
    <text
      x={QUESTION.x + left_shift}
      y={QUESTION.y}
      class="img-q-icon"
      style:font-size={QUESTION.font + "px"}>?</text>
  {/if}
</svg>
