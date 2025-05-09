<script module>
  import { t } from "../../lib/i18n";
</script>

<script>
  let { disabled = false, style = "" } = $props();
</script>

<button onclick={() => window.location.reload()} {disabled} {style}
  >{$t("sidebar-apply-button")}</button>
