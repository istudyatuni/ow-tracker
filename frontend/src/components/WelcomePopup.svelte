<script module>
  import FileUpload from "./atoms/FileUpload.svelte";
  import HideSpoilers from "./atoms/HideSpoilers.svelte";
  import HideDlc from "./atoms/HideDlc.svelte";
  import Popup from "./Popup.svelte";

  import { t } from "../lib/i18n";
  import { SAVE_FOUND, SAVE_FOUND_CATEGORIES, SETTINGS } from "../lib/stores";
  import { renderSnippet } from "../lib/utils";
  import { CATEGORY } from "../lib/categories";
  import { KEYS_COUNT } from "../lib/saves";
</script>

<script>
  let file_uploaded = $state(false);
  let save_complete_percent = $state(0);

  /** @param {Set} opened_facts */
  function handle_file_uploaded(opened_facts) {
    file_uploaded = true;
    save_complete_percent = (opened_facts.size / KEYS_COUNT) * 100;
  }
</script>

{#snippet game_name()}
  <a href="https://www.mobiusdigitalgames.com/outer-wilds.html" class="game"
    >Outer Wilds</a>
{/snippet}

<Popup>
  <h4 class="center">
    {@html $t("welcome-popup-header", { game: renderSnippet(game_name) })}
  </h4>
  {#if $SAVE_FOUND && !file_uploaded}
    <p class="center">{$t("welcome-popup-opening-save")}</p>
  {/if}
  <p>
    {#if file_uploaded}
      {$t("welcome-popup-save-file-approx-progress", {
        percent: save_complete_percent,
      })}
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
      class="mono orbital-canon"
      onclick={() => {
        SETTINGS.set("welcome_popup_done", true);
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
