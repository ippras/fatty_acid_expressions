# Introduction

Triacylglycerols

Lipids are more than just an energy source or building blocks for cellular membranes. They represent a complex language through which the body communicates its metabolic status, health, environmental adaptation, and nutritional quality. With the rapid advancement of lipidomics and nutrition science, the volume of data regarding fatty acids and their derivatives has grown exponentially. However, this abundance of data brings a challenge of systematization: how do we organize hundreds of individual molecules, their groups, and complex biomarkers (indices) into a single, logically consistent framework?

The book **"Lipid expressions"** offers an elegant solution to this problem. It is not merely a reference guide, but a rigorous mathematical and logical framework that translates the biological diversity of lipids into the language of clear formulas and expressions. 

The architecture of this book is built on a bottom-up principle—moving from the simple to the complex. We have divided all lipid expressions into three fundamental components: **Primitive**, **Sum**, and **Ratio**. This structure allows for the algorithmic calculation of any nutritional or metabolic index, no matter how complex.

## The Architecture of Lipid expressions

### Primitive

These are the basic "building blocks" of lipidomics—individual molecules that cannot be decomposed into simpler components within our framework. 
In the **Fatty Acids** section, primitives are specific acids with precise indications of carbon chain length and double bond positions: palmitic ($\ce{C{16:0}}$), oleic ($C18:1(\omega-9)$), eicosapentaenoic ($C20:5(\omega-3)$), and others. Primitives represent the raw data obtained directly from chromatographic analysis.

### Sum

The second level of abstraction. In biology and medicine, we rarely evaluate each fatty acid in isolation; more often, we are interested in their classes. **Sum** expressions group primitives based on specific classification criteria:
- **By chain length**: short chain (SCFA), medium chain (MCFA), long chain (LCFA), and very long chain (VLCFA) fatty acids.
- **By count of unsaturated bonds**: saturated (SFA), monounsaturated (MUFA), and polyunsaturated (PUFA).
- **By offset of the first unsaturated**: The well-known Omega -3, Omega -6, and Omega -9 families.
- **By pattern and parity**: Conjugated and trans fatty acids.
- **Specific combinations**: For example, the sum of the most important marine fatty acids (EPA + DHA).

Sums transform an array of raw data into meaningful biological categories.

### Ratio

The highest level of abstraction, where mathematics meets physiology. **Ratio** expressions demonstrate the relationships between primitives and/or sums. This is where biomarkers are born, evaluating food quality or metabolic states. This section is divided into two logical branches:
- *Nutritional*: Evaluate the impact of dietary lipids on human health. This includes the Atherogenic Index, Thrombogenic Index, Fish Lipid Quality, Health-Promoting Index, and others.
- *Metabolic*: Assess the activity of the body's endogenous enzyme systems, such as elongases (Elongase Index) and desaturases (Delta 9 Desaturase Index), as well as the Kinetic Activity Index.

### Who is this book for?

"Lipids Expressions" is designed for researchers in lipidomics, food chemistry, nutrition, and bioinformatics. Software developers creating laboratory analysis tools will find ready-to-use logic for writing algorithms. Scientists and medical professionals will gain a clear, structured reference base for health and metabolic indices. 
