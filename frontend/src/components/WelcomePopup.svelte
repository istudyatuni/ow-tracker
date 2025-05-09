<script module>
  import FileUpload from "@/components/atoms/FileUpload.svelte";
  import HideSpoilers from "@/components/atoms/HideSpoilers.svelte";
  import HideDlc from "@/components/atoms/HideDlc.svelte";
  import Popup from "@/components/Popup.svelte";

  import { t } from "@/lib/i18n";
  import {
    OPENED_FACTS_COUNT,
    reset_selected_categories,
    SAVE_FOUND,
    SAVE_FOUND_CATEGORIES,
    SESSION_SETTINGS,
  } from "@/lib/stores";
  import { renderSnippet } from "@/lib/utils";
  import { CATEGORY } from "@/lib/categories";
  import { KEYS_COUNT } from "@/lib/saves";

  /** @param {number} opened */
  function count_complete_percent(opened) {
    return (opened / KEYS_COUNT) * 100;
  }
</script>

<script>
  let file_uploaded = $state(false);
  let save_complete_percent = $derived(
    count_complete_percent($OPENED_FACTS_COUNT),
  );

  /** @param {Set} opened_facts */
  function handle_file_uploaded(opened_facts) {
    file_uploaded = true;
    save_complete_percent = count_complete_percent(opened_facts.size);
    reset_selected_categories();
  }
</script>

{#snippet game_name()}
  <a href="https://www.mobiusdigitalgames.com/outer-wilds.html" class="game"
    >Outer Wilds</a>
{/snippet}

{#snippet progress()}
  {$t("welcome-popup-save-file-approx-progress", {
    percent: save_complete_percent,
  })}
{/snippet}

<Popup>
  <h4 class="center">
    {@html $t("welcome-popup-header", { game: renderSnippet(game_name) })}
  </h4>
  {#if $SAVE_FOUND && !file_uploaded}
    <p class="center">
      {$t("welcome-popup-opening-save")}
      {@render progress()}
    </p>
  {/if}
  <p>
    {#if file_uploaded}
      {@render progress()}
    {:else}
      {$t("welcome-popup-upload-save-file")}:
    {/if}
    <FileUpload upload={handle_file_uploaded} />
  </p>
  {#if !file_uploaded}
    <p>
      {#if $SAVE_FOUND}
        {$t("welcome-popup-launch-save-map")}:
      {:else}
        {$t("welcome-popup-launch-full-map")}:
      {/if}
    </p>
  {/if}
  <HideSpoilers />
  <br />
  {#if $SAVE_FOUND_CATEGORIES.has(CATEGORY.STRANGER) && !file_uploaded}
    <HideDlc />
    <br />
  {/if}
  <div class="launch">
    <button
      class="mono green"
      onclick={() => {
        SESSION_SETTINGS.set("welcome_popup_done", true);
        window.location.reload();
      }}>{$t("welcome-popup-launch-button")} --|-..|-.</button>
  </div>
</Popup>

<style lang="scss">
  .center {
    text-align: center;
  }
  .game {
    color: orange;
    font-weight: 700;
    text-decoration: underline;
  }
  .launch {
    display: flex;
    width: 100%;

    & > button {
      margin: 1em auto auto;
    }
  }
</style>
