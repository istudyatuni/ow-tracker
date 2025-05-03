<script>
  import { onMount } from "svelte";

  import "leaflet/dist/leaflet.css";
  import L from "leaflet";

  import { coord_to_leaflet, generate_all_svg } from "./lib/draw";

  /** @type {import('leaflet').Map} */
  let map;

  onMount(async () => {
    map = L.map("map", {
      center: [250, 250],
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
      maxBounds: [coord_to_leaflet(-1500, -2200), coord_to_leaflet(4000, 2300)],
    }).on("click", (e) => {
      // @ts-ignore
      let id = e.originalEvent.target.id;
      if (id !== "map") {
        alert(`Clicked ${id}`);
      }
    });

    let { cards, rumors } = await generate_all_svg();
    for (let { svg, coords } of cards) {
      L.svgOverlay(svg, coords).addTo(map);
    }
    for (let { svg, coords } of rumors) {
      L.svgOverlay(svg, coords, {
        pane: "mapPane",
      }).addTo(map);
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
