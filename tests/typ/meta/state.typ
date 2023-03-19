// Test state.

---
#let s = state("hey", "a")
#let double(it) = 2 * it

#s.update(double)
#s.update(double)
$ 2 + 3 $
#s.update(double)

Is: #s.display(),
Was: #locate(id => {
  let it = query(math.equation, id).first()
  s.at(it.id())
}).

---
#set page(width: 200pt)
#set text(8pt)

#let ls = state("lorem", lorem(1000).split("."))
#let loremum(count) = {
  ls.display(list => list.slice(0, count).join(".").trim() + ".")
  ls.update(list => list.slice(count))
}

#let fs = state("fader", red)
#let trait(title) = block[
  #fs.display(color => text(fill: color)[
    *#title:* #loremum(1)
  ])
  #fs.update(color => color.lighten(30%))
]

#trait[Boldness]
#trait[Adventure]
#trait[Fear]
#trait[Anger]
