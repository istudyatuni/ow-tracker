<script>
  /** @import {Map} from  "leaflet" */

  import { get } from "svelte/store";

  import "leaflet/dist/leaflet.css";
  import L from "leaflet";

  import { generate_all_svg } from "@/lib/draw";
  import { MAP_SIZE } from "@/lib/stores";
  import { bounds_center, map_bounds_to_leaflet } from "@/lib/leaflet";
  import Leaflet from "@/Leaflet.svelte";
  import MapArrow from "@/components/MapArrow.svelte";
  import MapCard from "@/components/MapCard.svelte";

  /** @type {Map} */
  let map;

  /** @type {import("leaflet").MapOptions} */
  let mapOptions = {
    zoom: -2,
    minZoom: -2,
    maxZoom: 2,
    zoomDelta: 0.5,
    // fix zoomDelta not work in chrome
    wheelPxPerZoomLevel: 80,
    crs: L.CRS.Simple,
    attributionControl: false,
    zoomControl: false,
    maxBounds: map_bounds_to_leaflet(get(MAP_SIZE)),
  };
</script>

<div class="map">
  {#await generate_all_svg()}
    Loading
  {:then generator}
    <Leaflet
      {map}
      bounds={map_bounds_to_leaflet($MAP_SIZE)}
      center={bounds_center($MAP_SIZE)}
      options={mapOptions}>
      {#each generator() as { options, coords: bounds, pane }}
        {#if options.is_arrow}
          <MapArrow {...options} {bounds} {pane} />
        {:else}
          <MapCard {...options} {bounds} {pane} />
        {/if}
      {/each}
    </Leaflet>
  {/await}
</div>

<style lang="scss">
  .map {
    width: 100%;
    height: 100vh;

    & :global(.leaflet-container) {
      background: var(--bg);
    }
  }
</style>
