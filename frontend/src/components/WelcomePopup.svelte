<script module>
  import Popup from "./Popup.svelte";

  import { t } from "../lib/i18n";
  import { renderSnippet } from "../lib/utils";
  import FileUpload from "./atoms/FileUpload.svelte";
  import HideSpoilers from "./atoms/HideSpoilers.svelte";
  import HideDlc from "./atoms/HideDlc.svelte";
  import { SAVE_FOUND, SAVE_FOUND_CATEGORIES, SETTINGS } from "../lib/stores";
</script>

{#snippet game_name()}
  <a href="https://www.mobiusdigitalgames.com/outer-wilds.html" class="game"
    >Outer Wilds</a>
{/snippet}

<Popup>
  <h4 class="center">
    {@html $t("welcome-popup-header", { game: renderSnippet(game_name) })}
  </h4>
  {#if $SAVE_FOUND}
    <p class="center">{$t("welcome-popup-opening-save")}</p>
  {/if}
  <p>
    {$t("welcome-popup-upload-save-file")}:
    <FileUpload />
  </p>
  <p>
    {#if $SAVE_FOUND}
      {$t("welcome-popup-launch-save-map")}:
    {:else}
      {$t("welcome-popup-launch-full-map")}:
    {/if}
  </p>
  <HideSpoilers />
  <br />
  {#if $SAVE_FOUND_CATEGORIES.has("stranger")}
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
