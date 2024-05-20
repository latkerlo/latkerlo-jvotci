For most input strings, the default settings should give you the same answer as
[Camxes: Standard](http://lojban.github.io/ilmentufa/camxes.html) and 
[JVS](https://jbovlaste.lojban.org) (when you search for a string that is not
a word in the dictionary). The following is a list of types of cases where the
results produced by *latkerlo jvotci* are intentionally different from one or
both of them. In each case, *latkerlo jvotci* implements the morphology the way
I believe we as Lojbanists "should" want it to work. However, I would be 
willing to make modifications to conform to community consensus if further
discussion were to take place.

If you find a difference that isn't documented here, that may be a bug! It
would be great if you could leave an issue or otherwise get in touch to let me 
know.

<table>
  <tr>
    <th></th>
    <th>default settings</th>
    <th>JVS</th>
    <th>camxes: standard</th>
    <th></th>
  </tr>
  <tr>
    <td>kerlyspa'ykerlo</td>
    <td>✅ Brivla</td>
    <td>❌ Not Brivla</td>
    <td>❌ Not Brivla</td>
    <td rowspan="2">CVV, CVhV, and CCV forms are allowed on their own between 
y-hyphens.</td>
  </tr>
  <tr>
    <td>kerlyspaspa'ykerlo</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
  </tr>
  <tr>
    <td>spa'ykerlo</td>
    <td>✅ Brivla</td>
    <td>❌ Not Brivla</td>
    <td>❌ Not Brivla</td>
    <td rowspan="3">For initial CCV, this is more consistent with the rules 
when no y-hyphen is present</td>
  </tr>
  <tr>
    <td>spaspa'ykerlo</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
  </tr>
  <tr>
    <td>toispa'ykerlo</td>
    <td>❌ Not Brivla</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
  </tr>
  <tr><td colspan="5"></td></tr>
  <tr>
    <td>briiymle</td>
    <td>✅ Brivla</td>
    <td>❌ Not Brivla</td>
    <td>✅ Brivla</td>
    <td rowspan="3">zi'evla short rafsi can end with an on-glide.</td>
  </tr>
  <tr>
    <td>plukauaiymle</td>
    <td>✅ Brivla</td>
    <td>❌ Not Brivla</td>
    <td>✅ Brivla</td>
  </tr>
  <tr>
    <td>plukauai'ymle</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
  </tr>
  <tr><td colspan="5"></td></tr>
  <tr>
    <td>rai'y'ismu</td>
    <td>✅ Brivla</td>
    <td>❌ Not Brivla</td>
    <td>❌ Not Brivla</td>
    <td rowspan="2">In what world is "torai'y'ismu" a word?</td>
  </tr>
  <tr>
    <td>torai'y'ismu</td>
    <td>❌ Not Brivla</td>
    <td>✅ Brivla</td>
    <td>✅ Brivla</td>
  </tr>
  <tr><td colspan="5"></td></tr>
  <tr>
    <td>toiysmu</td>
    <td>❌ Not Brivla</td>
    <td>❌ Not Brivla</td>
    <td>✅ Brivla</td>
    <td>Egregioius.</td>
  </tr>
</table>