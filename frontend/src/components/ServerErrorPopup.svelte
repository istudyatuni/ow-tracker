<script module>
  import Popup from "@/components/Popup.svelte";

  import { t } from "@/lib/i18n";
</script>

<script>
  function reset_profile() {
    window.location.hash = "";
    window.location.reload();
  }
</script>

<Popup>
  <div class="center">
    <div>{$t("server-error-popup")}</div>
    <div class="mtop">
      <button onclick={reset_profile}
        >{$t("sidebar-show-full-map-button")}</button>
    </div>
  </div>
</Popup>

<style lang="scss">
  .center > * {
    margin: auto;
    width: fit-content;
  }
  .mtop {
    margin-top: 1em;
  }
</style>
