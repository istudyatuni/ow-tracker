<script>
  import { onMount } from "svelte";
  import { get } from "svelte/store";

  import "leaflet/dist/leaflet.css";
  import L from "leaflet";

  import { coord_to_leaflet, generate_all_svg } from "./lib/draw";
  import {
    close_fact,
    LOADING,
    MAP_SIZE,
    open_fact,
    OPENED_FACT,
  } from "./lib/stores";

  const MAP_PAD = 3000;

  /** @type {import('leaflet').Map} */
  let map;

  function map_bounds_to_leaflet(bounds) {
    return [
      coord_to_leaflet(bounds[0][0] - MAP_PAD, bounds[0][1] - MAP_PAD),
      coord_to_leaflet(bounds[1][0] + MAP_PAD, bounds[1][1] + MAP_PAD),
    ];
  }
  /**
   * @param  {number[][]} bounds
   * @return {import('leaflet').LatLngTuple}
   */
  function bounds_center(bounds) {
    let b = [
      coord_to_leaflet(bounds[0][0], bounds[0][1]),
      coord_to_leaflet(bounds[1][0], bounds[1][1]),
    ];
    return [b[0][0] / 2 + b[1][0] / 2, b[0][1] / 2 + b[1][1] / 2];
  }

  onMount(async () => {
    let bounds = get(MAP_SIZE);

    map = L.map("map", {
      center: bounds_center(bounds),
      zoom: -2,
      minZoom: -2,
      maxZoom: 2,
      zoomDelta: 0.5,
      // fix zoomDelta not work in chrome
      wheelPxPerZoomLevel: 80,
      crs: L.CRS.Simple,
      attributionControl: false,
      zoomControl: false,
      maxBounds: map_bounds_to_leaflet(bounds),
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

    for await (let { svg, coords, pane } of generate_all_svg()) {
      L.svgOverlay(svg, coords, { pane }).addTo(map);
    }

    LOADING.set(null);
  });
</script>

<div id="map"></div>

<style>
  #map {
    width: 100%;
    height: 100vh;
    background: var(--bg);
  }
</style>
