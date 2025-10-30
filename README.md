# xml-skim

Skims over (possibly massive) XML files in seconds.

- Spits out statistics of how many nodes exist at
each hierarchy point.
- Collects stats on text nodes as well


## Installation

Just get the binary

```sh
cargo binstall xml-skim
```

For development use

```sh
cargo add xml-skim
```


## Usage

Just pass it xml files.

```sh
Usage: xml-skim [file1.xml, file2.xml ...]
```

For a simple xml it spits the tags and text nodes in XPath format.

```sh
$ cat > file1.xml <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<Descriptions>
  <Version date="2025-11-11" />
  <DescriptionList>
    <Description>
      <Type>Gene</Type>
      <Name>SHH</Name>
      <Term>Sonic Hedgehog</Term>
    </Description>
    <Description>
      <![CDATA[
      Location at 122341
      ]]>
      <Type>
        Exon
      </Type>
      <!-- location in proper xml tag -->
      <Loc>122341</Loc>
    </Description>
  </DescriptionList>
</Descriptions>
EOF
$ cat > file2.xml <<EOF
<flimsy>
  <description>
  Gene: SHH
  </description>
  <description gene="BRAC" />
  <description>
  Gene: MTOR
  </description>
</flimsy>
EOF
$ xml-skim file1.xml file2.xml
File: file1.xml
/Descriptions : 1
/Descriptions/Version : 1
/Descriptions/DescriptionList : 1
/Descriptions/DescriptionList/Description : 2
/Descriptions/DescriptionList/Description/Type : 2
/Descriptions/DescriptionList/Description/Type/text() : 2
/Descriptions/DescriptionList/Description/Name : 1
/Descriptions/DescriptionList/Description/Name/text() : 1
/Descriptions/DescriptionList/Description/Term : 1
/Descriptions/DescriptionList/Description/Term/text() : 1
/Descriptions/DescriptionList/Description/Loc : 1
/Descriptions/DescriptionList/Description/Loc/text() : 1
File: file2.xml
/flimsy : 1
/flimsy/description : 3
/flimsy/description/text() : 2
```

The output are a valid XPath followed by the count.
The output is designed to be `grep`able,
for example `text()` is not a valid xml tag name
as it is a defined XPath function.


## About

Copyright (c) 2025 Michal Grochmal

`xml-skim` is distributed under the MIT license.
See the LICENSE file for details.
