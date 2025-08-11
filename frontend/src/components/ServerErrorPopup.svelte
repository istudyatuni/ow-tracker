<script module>
  import Popup from "@/components/Popup.svelte";

  import { t } from "@/lib/i18n";
  import { PROFILE_LOADING_STATUS } from "@/lib/saves";
  import { PROFILE_SAVE_LOADING_STATUS } from "@/lib/stores";
</script>

<script>
  function reset_profile() {
    window.location.hash = "";
    window.location.reload();
  }
</script>

<Popup>
  <div class="center">
    <div>
      {#if $PROFILE_SAVE_LOADING_STATUS == PROFILE_LOADING_STATUS.failed}
        {$t("profile-load-error-popup-failed")}
      {:else if $PROFILE_SAVE_LOADING_STATUS == PROFILE_LOADING_STATUS.not_found}
        {$t("profile-load-error-popup-not-found")}
      {:else if $PROFILE_SAVE_LOADING_STATUS == PROFILE_LOADING_STATUS.unavailable}
        {$t("profile-load-error-popup-server-unavailable")}
      {/if}
    </div>
    <div>
      <button onclick={reset_profile}
        >{$t("sidebar-show-full-map-button")}</button>
    </div>
  </div>
</Popup>

<style lang="scss">
  .center {
    & > * {
      margin: auto;
      width: fit-content;
    }
    & > *:has(+ div) {
      margin-bottom: 1em;
    }
  }
</style>
