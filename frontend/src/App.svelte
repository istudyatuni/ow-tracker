<script module>
  import FactsPanel from "./components/FactsPanel.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import Loading from "./components/Loading.svelte";
  import WelcomePopup from "./components/WelcomePopup.svelte";
  import Map from "./Map.svelte";

  import { migrate_storage, OPENED_FACT, SETTINGS } from "./lib/stores";
  import { get_facts_for, has_more_to_explore } from "./lib/data";
  import { init_i18n } from "./lib/i18n";
</script>

<script>
  init_i18n();
  migrate_storage();

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
  {#if !$SETTINGS.welcome_popup_done}
    <WelcomePopup />
  {/if}
</main>

<style>
</style>
