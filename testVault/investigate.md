---
title: Investigating
type: scene
---

# You Look Around

The room is sparse but well-kept. You notice a journal on the desk and a key under the bed.

**Old Keeper**: "Be careful, there are secrets in this place."
**You**: "What kind of secrets?"

[[journal|Read the journal]](player.curiosity += 15)
[[key|Take the key]](player.inventory = "key"; player.boldness += 10)
{if: player.curiosity > 20}[[secret|Find a secret passage]](player.wisdom = 100)
[[start|Go back]](player.curiosity -= 5)
