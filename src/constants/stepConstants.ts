import { SpecifiedFitParamsType } from "../types/data.types"

export const stepsVariant=[
	{value:3, label:"3мм", area:0.071, default:false},
	{value:4, label:"4мм", area:0.126, default:false},
	{value:5, label:"5мм", area:0.196, default:false},
	{value:6, label:"6мм", area:0.283, default:true},
	{value:8, label:"8мм", area:0.503, default:true},
	{value:10, label:"10мм", area:0.785, default:true},
	{value:12, label:"12мм", area:1.31, default:true},
	{value:14, label:"14мм", area:1.54, default:true},
	{value:16, label:"16мм", area:2.01, default:true},
	{value:18, label:"18мм", area:2.54, default:true},
	{value:20, label:"20мм", area:3.14, default:true},
	{value:22, label:"22мм", area:3.8, default:true},
	{value:25, label:"25мм", area:4.91, default:true},
	{value:28, label:"28мм", area:6.16, default:true},
	{value:32, label:"32мм", area:8.01, default:true},
	{value:36, label:"36мм", area:10.18, default:false},
	{value:40, label:"36мм", area:12.56, default:false},
	{value:45, label:"45мм", area:15, default:false},
	{value:50, label:"50мм", area:19.63, default:false},
	{value:55, label:"55мм", area:23.76, default:false},
]
export const stepVariantsToState:SpecifiedFitParamsType[] = stepsVariant.map(el=>{
	return {
		diameter:el.value,
		price:null,
		area:el.area,
		isSpecified:false,
		default:el.default
	}
})