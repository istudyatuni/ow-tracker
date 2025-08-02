<script module>
  import Debug from "@/components/Debug.svelte";
  import FactsPanel from "@/components/FactsPanel.svelte";
  import Sidebar from "@/components/Sidebar.svelte";
  import Loading from "@/components/Loading.svelte";
  import Popup from "@/components/Popup.svelte";
  import WelcomePopup from "@/components/WelcomePopup.svelte";
  import Map from "@/Map.svelte";

  import {
    migrate_storage,
    OPENED_FACT,
    SAVE_EMPTY,
    SAVE_FOUND_CATEGORIES,
    SESSION_SETTINGS,
    SELECTED_CATEGORIES,
    SETTINGS,
    LANGUAGE,
  } from "@/lib/stores";
  import { get_facts_for } from "@/lib/data";
  import { init_i18n, t } from "@/lib/i18n";
  import { NEED_EXTENDED_SPOILER_FONT } from "@/lib/language";
</script>

<script>
  init_i18n();
  migrate_storage();

  let hide_categories = $derived(
    Object.fromEntries(
      Object.entries($SELECTED_CATEGORIES).map(([category, selected]) => [
        "hide-" + category,
        !selected,
      ]),
    ),
  );
  let facts = $derived(
    $OPENED_FACT
      ? get_facts_for($OPENED_FACT, $SETTINGS.show_ignored_facts)
      : [],
  );
  let is_map_empty = $derived(
    // if save has no opened cards
    $SAVE_EMPTY ||
      // or none of the categories found in the save are selected
      $SAVE_FOUND_CATEGORIES.intersection(
        new Set(
          Object.entries($SELECTED_CATEGORIES)
            .filter(([_, v]) => v)
            .map(([v, _]) => v),
        ),
      ).size === 0,
  );
</script>

<main
  class={hide_categories}
  class:hide-spoilers={$SETTINGS.hide_spoilers}
  class:extended-spoiler-font={NEED_EXTENDED_SPOILER_FONT.has($LANGUAGE)}>
  <Sidebar />
  <Map />
  <FactsPanel {facts} />
  <Loading />
  {#if !$SESSION_SETTINGS.welcome_popup_done}
    <WelcomePopup />
  {:else if is_map_empty}
    <Popup>{$t("map-empty-popup")}</Popup>
  {/if}

  {#if import.meta.env.DEV}
    <Debug />
  {/if}
</main>
