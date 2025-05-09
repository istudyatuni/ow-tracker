<script module>
  import { SETTINGS } from "@/lib/stores";
  import { t } from "@/lib/i18n";
</script>

<script>
  let { onchange = () => {} } = $props();
</script>

<label>
  <input
    type="checkbox"
    name="show-unexplored"
    class="orange"
    {onchange}
    bind:checked={$SETTINGS.show_ignored_facts} />
  {$t("show-unexplored-checkbox")}
</label>
