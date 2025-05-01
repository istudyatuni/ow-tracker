<script>
  import { onMount } from "svelte";

  import "leaflet/dist/leaflet.css";
  import L from "leaflet";

  import { make_rumor_arrow } from "./arrow";
  import { CARD_HEIGHT, CARD_WIDTH, make_card_svg } from "./card";
  import { detect_language } from "./language";
  import { to_data_url } from "./dataurl";
  import { CURIOSITY } from "./info";

  const DEFAULT_MULT = 1;
  const SMALL_MULT = 0.4;

  const HIDE_CURIOSITIES = [CURIOSITY.INVISIBLE_PLANET];
  // pretend that save file was loaded
  const TEST_SAVE = true;

  /** @type {import('leaflet').Map} */
  let map;

  /** @return {import('leaflet').LatLngTuple} */
  function coord_to_leaflet(x, y) {
    const Y_CONV = 1;
    return [y * Y_CONV, x];
  }

  onMount(async () => {
    let save_data = (await (await fetch("test-save.json")).json())
      .shipLogFactSaves;

    // todo: not sure if read and newlyRevealed affect showing
    // || fact.read || fact.newlyRevealed
    let is_fact_opened = (fact) => fact.revealOrder >= 0;

    // which facts in save are opened
    let opened_facts = new Set();
    for (let [id, fact] of Object.entries(save_data)) {
      if (is_fact_opened(fact)) {
        opened_facts.add(id);
      }
    }

    // load ids data and rumors source ids
    let library = {};
    /**
     * rumor's source id -> [{entry_id, rumor_id}]
     * @type {Object.<string, Object.<string, string>[]>}
     */
    let sources = {};
    let entries_data = await (await fetch("entries.json")).json();
    // opened cards ids
    let opened_cards = new Set();
    // cards ids where img is opened
    let opened_card_imgs = new Set();

    function handle_entries(entries) {
      for (let e of entries || []) {
        library[e.id] = {
          curiosity: e.curiosity,
        };

        // fill opened_cards and opened_card_imgs
        for (let fact of e?.facts?.explore || []) {
          if (opened_facts.has(fact.id)) {
            opened_cards.add(e.id);
            opened_card_imgs.add(e.id);
          }
        }
        for (let fact of e?.facts?.rumor || []) {
          if (opened_facts.has(fact.id)) {
            opened_cards.add(e.id);
          }
        }
        // fill source_ids
        for (let fact of e?.facts?.rumor || []) {
          if (fact.source_id === undefined) {
            continue;
          }

          let obj = {
            entry_id: e.id,
            rumor_id: fact.id,
          };
          if (sources[fact.source_id] !== undefined) {
            sources[fact.source_id].push(obj);
          } else {
            sources[fact.source_id] = [obj];
          }
        }
        handle_entries(e.entries);
      }
    }
    handle_entries(entries_data);

    // load coordinates and images
    let entries = {};
    let coordinates_data = await (await fetch("library.json")).json();
    for (let e of coordinates_data.entries) {
      entries[e.id] = {
        coordinates: coord_to_leaflet(e.cardPosition.x, e.cardPosition.y),
        sprite: opened_card_imgs.has(e.id)
          ? "/sprites/" + e.spritePath.replace("png", "jpg")
          : null,
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

      if (!TEST_SAVE && HIDE_CURIOSITIES.includes(library[id]?.curiosity)) {
        continue;
      }
      if (TEST_SAVE && !opened_cards.has(id)) {
        continue;
      }

      let img = await (async () => {
        if (e.sprite === null) {
          return null;
        }
        let img = await (await fetch(e.sprite)).blob();
        return await to_data_url(img);
      })();
      let svg = make_card_svg(
        id,
        tr[id].replaceAll("@@", "<br/>").replaceAll("$$", "-<br/>"),
        img,
        colors?.color,
        colors?.highlight,
      );
      L.svgOverlay(svg, [c, bounds]).addTo(map);
    }

    for (let [source_id, entry_ids] of Object.entries(sources)) {
      if (
        !TEST_SAVE &&
        HIDE_CURIOSITIES.includes(library[source_id]?.curiosity)
      ) {
        continue;
      }
      if (TEST_SAVE && !opened_cards.has(source_id)) {
        continue;
      }
      for (let { entry_id, rumor_id } of entry_ids) {
        if (TEST_SAVE && !opened_facts.has(rumor_id)) {
          continue;
        }
        let svg = make_rumor_arrow(
          rumor_id,
          centers[source_id],
          centers[entry_id],
        );
        L.svgOverlay(svg, [centers[source_id], centers[entry_id]], {
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
