cmd "datapack list"
DP_ENABLED=0
for i in $(seq 1 30); do
  DP_ENABLED=$(grep -oE "\[(file/)?(terralith|tectonic|incendium|stellarity)" server-stdout.log | wc -l)
  [ "$DP_ENABLED" -ge 4 ] && break
  sleep 2
done
# AG-395 fix (blocker #3): grep -c counts LINES; Paper lists all 4 packs on ONE line
# ("There are 7 data pack(s) enabled: [vanilla], [file/bukkit], [file/terralith.zip (world)]...")
# -> enabled-markers=1 false-fail. Count OCCURRENCES (grep -o | wc -l); run-36794417339 proof.
log "G-DATAPACKS enabled-markers=$DP_ENABLED (expect 4)"
[ "$DP_ENABLED" -ge 4 ] || { log "G-DATAPACKS FAIL (datapacks not all enabled)"; FAIL=1; }
