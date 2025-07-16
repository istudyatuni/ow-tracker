<svelte:options namespace="svg" />

<script module>
  import { getContext, onMount } from "svelte";
  import * as L from "leaflet";

  import { STROKE } from "@/lib/arrow";

  // arrowhead with width = 30 and height = 45
  const ARROW = {
    path: "M 0,0 L 0,30 L 15,45 L 30,30 L 30,0 L 15,15 Z",
    cx: 15,
    cy: 22.5,
  };
</script>

<script>
  let { id, center1, center2, bounds, pane, category_class } = $props();

  let [x1, y1] = center1;
  let [x2, y2] = center2;

  let rect_width = Math.abs(y2 - y1);
  let rect_height = Math.abs(x2 - x1);

  let dx = x2 - x1;
  let dy = y2 - y1;

  // todo: calculate not just middle between centers, but center between
  // intersections with cards edges
  let [cx, cy] = [rect_width / 2, rect_height / 2];
  let rad = Math.atan2(y1 - y2, x1 - x2);
  let deg = (rad * 180) / Math.PI;

  let self = null;

  onMount(() =>
    L.svgOverlay(self, bounds, { pane }).addTo(getContext("map")()),
  );
</script>

<svg
  viewBox="0 0 {rect_width} {rect_height}"
  bind:this={self}
  class={category_class}>
  <!--
    when `dx * dy < 0` one coordinate increases, other decreases,
    so `dx` and `dy` has different signs
  -->
  {#if dx * dy < 0}
    <!-- top left -> bottom right -->
    <!-- prettier-ignore -->
    <line {id} x1="0" y1="0" x2={rect_width} y2={rect_height} class="arrow" stroke-width={STROKE} />
  {:else}
    <!-- bottom left -> top right -->
    <!-- prettier-ignore -->
    <line {id} x1="0" y1={rect_height} x2={rect_width} y2="0" class="arrow" stroke-width={STROKE} />
  {/if}

  <path
    d={ARROW.path}
    transform="translate({cx - ARROW.cx}, {cy -
      ARROW.cy}) rotate({deg}, {ARROW.cx}, {ARROW.cy})"
    class="arrow" />
</svg>
