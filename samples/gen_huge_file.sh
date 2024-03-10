#!/bin/bash

# Function to validate if the provided argument is a file
is_file() {
  [[ -f "$1" ]] && return 0
  return 1
}

# Check if a file is provided as an argument
if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <filename>"
  exit 1
fi

# Validate if the argument is a file
if ! is_file "$1"; then
  echo "Error: '$1' is not a file."
  exit 1
fi

cp template.xml "$1"

# Define the XML string
xml_string="            
<Ntry>
                <NtryRef>52198201</NtryRef>
                <Amt Ccy=\"USD\">10.00</Amt>
                <CdtDbtInd>CRDT</CdtDbtInd>
                <RvslInd>true</RvslInd>
                <Sts>BOOK</Sts>
                <BookgDt>
                    <DtTm>2023-10-01T13:37:14.000</DtTm>
                </BookgDt>
                <ValDt>
                    <Dt>2023-10-01</Dt>
                </ValDt>
                <BkTxCd>
                    <Prtry>
                        <Cd>ACH Credit Reject</Cd>
                    </Prtry>
                </BkTxCd>
                <NtryDtls>
                    <TxDtls>
                        <Refs>
                            <MsgId>GSNULXSKMMJ479NMKS</MsgId>
                            <AcctSvcrRef>B20092800002225</AcctSvcrRef>
                            <PmtInfId>RP/GS/CTFILERP0002/CTBA0003</PmtInfId>
                            <EndToEndId>GSGWGDNCTAHQM8</EndToEndId>
                        </Refs>
                        <AmtDtls>
                            <InstdAmt>
                                <Amt Ccy=\"USD\">10.00</Amt>
                            </InstdAmt>
                            <TxAmt>
                                <Amt Ccy=\"USD\">10.00</Amt>
                                <CcyXchg>
                                    <TrgtCcy>USD</TrgtCcy>
                                </CcyXchg>
                            </TxAmt>
                        </AmtDtls>
                        <RmtInf>
                            <Ustrd>Sample Unstructured Remittance 123</Ustrd>
                        </RmtInf>
                        <RtrInf>
                            <AddtlInf>Status changed to REJECTED : REJECT REVERSAL</AddtlInf>
                        </RtrInf>
                    </TxDtls>
                </NtryDtls>
            </Ntry>
"

endString="
        </Stmt>
    </BkToCstmrStmt>
</Document>
"
# Escape newlines in the XML string for sed
escaped_xml_string=$(echo "$xml_string" | sed ':a;N;$!ba;s/\n/\\n/g')

# Insert the XML string into the file do this 5 times
for i in $(seq 1 5)
do
    str=$xml_string
    # Initialize strs array
    declare -a strs=()
# create an array of 100000 elements
    for i in $(seq 1 100000)
    do
    strs[${#strs[@]}]=${str}
    done
 
    IFS= eval 'result="${strs[*]}"'

    echo "${result}" >> "$1"
done
echo $endString >> "$1"

echo "XML string inserted successfully into '$1'."