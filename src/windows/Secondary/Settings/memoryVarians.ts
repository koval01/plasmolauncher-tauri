import { MemoryAllocation } from "@bindings/MemoryAllocation"

interface MemoryVariant {
    label: string,
    value: MemoryAllocation,
}

const memoryVariants: MemoryVariant[] = [
    {
        label: "1 ГБ",
        value: "_1G"
    },
    {
        label: "2 ГБ",
        value: "_2G"
    },
    {
        label: "4 ГБ",
        value: "_4G"
    },
    {
        label: "6 ГБ",
        value: "_6G"
    },
    {
        label: "8 ГБ",
        value: "_8G"
    },
]

export default memoryVariants

export type { MemoryVariant }