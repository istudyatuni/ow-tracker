<script>
  import { onMount } from "svelte";

  import "leaflet/dist/leaflet.css";
  import L from "leaflet";

  import { CARD_HEIGHT, CARD_WIDTH, make_card_svg } from "./card";

  const DEFAULT_MULT = 1;
  const SMALL_MULT = 0.5;

  let map;

  /** @return {import('leaflet').LatLngTuple} */
  function coord_to_leaflet(x, y) {
    const Y_CONV = 1;
    return [y * Y_CONV, x];
  }

  onMount(async () => {
    // load ids data
    let library = {};
    let entries_data = await (await fetch("entries.json")).json();
    function handle_entries(entries, depth = 1) {
      if (entries === undefined) {
        return;
      }
      for (let e of entries) {
        if (e.curiosity !== undefined) {
          library[e.id] = {
            curiosity: e.curiosity,
          };
        }
        handle_entries(e.entries, depth + 1);
      }
    }
    handle_entries(entries_data);

    // load coordinates and images
    let entries = {};
    let coordinates_data = await (await fetch("library.json")).json();
    for (let e of coordinates_data.entries) {
      entries[e.id] = {
        coordinates: coord_to_leaflet(e.cardPosition.x, e.cardPosition.y),
        sprite: "/sprites/" + e.spritePath.replace("png", "jpg"),
      };
    }

    // load parents map
    let parents = await (await fetch("parents.json")).json();

    // load translations
    let tr = await (await fetch("translations/english.json")).json();

    // load theme colors
    let theme = await (await fetch("theme.json")).json();

    map = L.map("map", {
      center: [250, 250],
      zoom: 0,
      minZoom: -2,
      maxZoom: 2,
      crs: L.CRS.Simple,
      attributionControl: false,
      zoomControl: false,
      // max/min in normal coordinates:
      // x: [-878, 3341.8005]
      // y: [-1577, 1707]
      maxBounds: [coord_to_leaflet(-1500, -2200), coord_to_leaflet(4000, 2300)],
    });

    let neutral_color = theme.neutral.color;
    for (let [id, e] of Object.entries(entries)) {
      // L.marker(e.coordinates).addTo(map).bindPopup(id);
      let c = e.coordinates;
      let [x, y] = c;

      let colors = theme[library[id]?.curiosity];

      let is_small = id in parents;
      let mult = is_small ? SMALL_MULT : DEFAULT_MULT;

      let img = await (await fetch(e.sprite)).blob();
      const reader = new FileReader();
      reader.readAsDataURL(img);
      reader.onloadend = () => {
        let svg = make_card_svg(
          tr[id].replaceAll("@@", "<br/>").replaceAll("$$", "-<br/>"),
          // @ts-ignore
          reader.result,
          colors?.color || neutral_color,
        );
        L.imageOverlay("data:image/svg+xml;utf8," + encodeURIComponent(svg), [
          c,
          [x - CARD_HEIGHT * mult, y + CARD_WIDTH * mult],
        ]).addTo(map);
      };
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
