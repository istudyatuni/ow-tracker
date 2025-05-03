<script module>
  // import svelteLogo from './assets/svelte.svg'
  // import viteLogo from '/vite.svg'

  import FactsPanel from "./components/FactsPanel.svelte";
  import Map from "./Map.svelte";

  import { OPENED_FACT } from "./lib/stores";
  import { get_facts_for } from "./lib/data";
</script>

<script>
  let facts = $derived($OPENED_FACT ? get_facts_for($OPENED_FACT) : []);
</script>

<main>
  <Map />
  <FactsPanel {facts} />
</main>

<style>
</style>
