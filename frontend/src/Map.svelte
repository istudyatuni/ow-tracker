<script>
  import { onMount } from "svelte";
  import { get } from "svelte/store";

  import "leaflet/dist/leaflet.css";
  import L from "leaflet";

  import { coord_to_leaflet, generate_all_svg } from "./lib/draw";
  import { close_fact, open_fact, OPENED_FACT } from "./lib/stores";

  const MAP_PAD = 3000;

  /** @type {import('leaflet').Map} */
  let map;

  onMount(async () => {
    map = L.map("map", {
      center: [250, 1000],
      zoom: -2,
      minZoom: -2,
      maxZoom: 2,
      // not work
      zoomDelta: 0.5,
      crs: L.CRS.Simple,
      attributionControl: false,
      zoomControl: false,
      // max/min in normal coordinates:
      // x: [-878, 3341.8005]
      // y: [-1577, 1707]
      maxBounds: [
        coord_to_leaflet(-900 - MAP_PAD, -1600 - MAP_PAD),
        coord_to_leaflet(3300 + MAP_PAD, 1700 + MAP_PAD),
      ],
    }).on("click", (e) => {
      // @ts-ignore
      let id = e.originalEvent.target.id;

      let cur_id = get(OPENED_FACT);
      if (id === cur_id) {
        close_fact();
        return;
      }

      if (id !== "map") {
        open_fact(id);
      } else {
        close_fact();
      }
    });

    let svgs = await generate_all_svg();
    for (let { svg, coords, pane } of svgs) {
      L.svgOverlay(svg, coords, { pane }).addTo(map);
    }
  });
</script>

<div id="map"></div>

<style>
  #map {
    width: 100%;
    height: 100vh;
    background: #02101b;
  }
</style>
