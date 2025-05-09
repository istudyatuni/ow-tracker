<script module>
  import FactsPanel from "./components/FactsPanel.svelte";
  import Sidebar from "./components/Sidebar.svelte";
  import Loading from "./components/Loading.svelte";
  import Popup from "./components/Popup.svelte";
  import WelcomePopup from "./components/WelcomePopup.svelte";
  import Map from "./Map.svelte";

  import {
    MAP_EMPTY,
    migrate_storage,
    OPENED_FACT,
    SESSION_SETTINGS,
  } from "./lib/stores";
  import { get_facts_for } from "./lib/data";
  import { init_i18n, t } from "./lib/i18n";
</script>

<script>
  init_i18n();
  migrate_storage();

  let facts = $derived($OPENED_FACT ? get_facts_for($OPENED_FACT) : []);
</script>

<main>
  <Sidebar />
  <Map />
  <FactsPanel {facts} />
  <Loading />
  {#if !$SESSION_SETTINGS.welcome_popup_done}
    <WelcomePopup />
  {:else if $MAP_EMPTY}
    <Popup>{$t("map-empty-popup")}</Popup>
  {/if}
</main>

<style>
</style>
