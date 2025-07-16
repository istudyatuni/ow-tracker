<!-- adapted from https://github.com/dimfeld/svelte-leaflet-demo/blob/master/full/src/map/Leaflet.svelte -->
<script module>
  import { setContext } from "svelte";
  import { get } from "svelte/store";

  import * as L from "leaflet";
  import "leaflet/dist/leaflet.css";

  import { close_fact, MAP_SIZE, open_fact, OPENED_FACT } from "@/lib/stores";
  import { bounds_center, map_bounds_to_leaflet } from "@/lib/leaflet";
</script>

<script>
  let {
    map,
    bounds,
    center,
    options = {},
    height = "100%",
    width = "100%",
    children,
  } = $props();

  setContext("map", () => map);

  function createLeaflet(node) {
    map = L.map(node, options)
      .fitBounds(bounds)
      .setView(center)
      .on("click", (e) => {
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

    MAP_SIZE.subscribe((bounds) => {
      map
        .fitBounds(map_bounds_to_leaflet(bounds))
        .setView(bounds_center(bounds));
    });

    return {
      destroy() {
        map.remove();
        map = undefined;
      },
    };
  }
</script>

<div style="height:{height};width:{width}" use:createLeaflet></div>

{@render children()}
