<script module>
  // import svelteLogo from './assets/svelte.svg'
  // import viteLogo from '/vite.svg'

  import FactsPanel from "./components/FactsPanel.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import Loading from "./components/Loading.svelte";
  import Popup from "./components/Popup.svelte";
  import Map from "./Map.svelte";

  import { OPENED_FACT, SAVE_FOUND } from "./lib/stores";
  import { get_facts_for } from "./lib/data";
</script>

<script>
  let facts = $derived($OPENED_FACT ? get_facts_for($OPENED_FACT) : []);
</script>

<main>
  <Sidebar />
  <Map />
  <FactsPanel {facts} />
  <Loading />
  <Popup text="Upload save file from menu" hidden={$SAVE_FOUND} />
</main>

<style>
</style>
