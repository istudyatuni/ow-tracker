<script>
  import { onMount } from "svelte";

  import "leaflet/dist/leaflet.css";
  import L from "leaflet";

  import { CARD_HEIGHT, CARD_WIDTH, make_card_svg } from "./card";
  import { detect_language } from "./language";
  import { to_data_url } from "./dataurl";
  import { CURIOSITY } from "./info";

  const DEFAULT_MULT = 1;
  const SMALL_MULT = 0.4;

  const HIDE_CURIOSITIES = [CURIOSITY.INVISIBLE_PLANET];

  /** @type {import('leaflet').Map} */
  let map;

  /** @return {import('leaflet').LatLngTuple} */
  function coord_to_leaflet(x, y) {
    const Y_CONV = 1;
    return [y * Y_CONV, x];
  }

  onMount(async () => {
    // load ids data and rumors source ids
    let library = {};
    /**
     * rumor source id -> [entry id]
     * @type {Object.<string, string[]>}
     */
    let source_ids = {};
    let entries_data = await (await fetch("entries.json")).json();
    function handle_entries(entries, depth = 1) {
      for (let e of entries || []) {
        if (e.curiosity !== undefined) {
          library[e.id] = {
            curiosity: e.curiosity,
          };
        }

        for (let rumor of e?.facts?.rumor || []) {
          if (rumor.source_id !== undefined) {
            if (source_ids[rumor.source_id] !== undefined) {
              source_ids[rumor.source_id].push(e.id);
            } else {
              source_ids[rumor.source_id] = [e.id];
            }
          }
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
    let lang = detect_language();
    let tr = await (await fetch(`translations/${lang}.json`)).json();

    // load theme colors
    let theme = await (await fetch("theme.json")).json();

    map = L.map("map", {
      center: [250, 250],
      zoom: 0,
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
    });
    map.on("click", (e) => {
      // @ts-ignore
      let id = e.originalEvent.target.id;
      if (id !== "map") {
        alert(`Clicked ${id}`);
      }
    });

    let centers = {};

    let neutral_theme = theme.neutral;
    for (let [id, e] of Object.entries(entries)) {
      let colors = theme[library[id]?.curiosity] || neutral_theme;

      let is_small = id in parents;
      let mult = is_small ? SMALL_MULT : DEFAULT_MULT;

      let c = e.coordinates;
      let [x, y] = c;
      let w = CARD_WIDTH * mult;
      let h = CARD_HEIGHT * mult;
      let bounds = [x + h, y + w];

      centers[id] = [x + h / 2, y + w / 2];

      if (HIDE_CURIOSITIES.includes(library[id]?.curiosity)) {
        continue;
      }

      let img = await (await fetch(e.sprite)).blob();
      let svg = make_card_svg(
        id,
        tr[id].replaceAll("@@", "<br/>").replaceAll("$$", "-<br/>"),
        await to_data_url(img),
        colors?.color,
        colors?.highlight,
      );
      L.svgOverlay(svg, [c, bounds]).addTo(map);
    }

    for (let [source_id, entry_ids] of Object.entries(source_ids)) {
      if (HIDE_CURIOSITIES.includes(library[source_id]?.curiosity)) {
        continue;
      }
      for (let entry_id of entry_ids) {
        L.polyline([centers[entry_id], centers[source_id]], {
          color: neutral_theme.color,
          pane: "mapPane",
        }).addTo(map);
      }
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
