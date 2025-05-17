<script module>
  import { t } from "@/lib/i18n";
  import { LOADING, LOADING_STAGE, LOADING_TOTAL } from "@/lib/stores";
</script>

<div class="block above-map-info" class:hidden={$LOADING === null}>
  <label>
    {$t("loading-prefix")}
    {$t(`loading-stage-${$LOADING_STAGE}`)}
    <progress value={$LOADING} max={$LOADING_TOTAL}></progress>
  </label>
</div>

<style lang="scss">
  $color: #3280ff;

  .block {
    position: absolute;
    top: 45%;
    left: 5%;
    right: 5%;

    height: auto;

    border: 2px $color solid;
    background-color: var(--bg);
    font-size: 1.3em;
    padding: 1em;

    @media screen and (max-width: 400px) {
      padding-left: 5%;
    }
    @media screen and (max-width: 800px) and (min-width: 400px) {
      padding-left: 10%;
    }
    @media screen and (max-width: 1500px) and (min-width: 800px) {
      padding-left: 20%;
    }
    @media screen and (min-width: 1500px) {
      padding-left: 30%;
    }
  }
  .hidden {
    display: none;
  }
</style>
