<script module>
  import FactsPanel from "./components/FactsPanel.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import Loading from "./components/Loading.svelte";
  import Popup from "./components/Popup.svelte";
  import Map from "./Map.svelte";

  import { OPENED_FACT, SAVE_FOUND } from "./lib/stores";
  import { get_facts_for, has_more_to_explore } from "./lib/data";
</script>

<script>
  let facts = $derived($OPENED_FACT ? get_facts_for($OPENED_FACT) : []);
  let more_to_explore = $derived(
    $OPENED_FACT ? has_more_to_explore($OPENED_FACT) : false,
  );
</script>

<main>
  <Sidebar />
  <Map />
  <FactsPanel {facts} {more_to_explore} />
  <Loading />
  <Popup text="Upload save file from menu" hidden={$SAVE_FOUND} />
</main>

<style>
</style>
