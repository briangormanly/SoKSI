# power law analysis scripts
import powerlaw
import csv 
import os 
import numpy as np 
import matplotlib.pyplot as plt 

np.seterr(divide='ignore', invalid='ignore')


data = np.genfromtxt('py-powerlaw-import.txt')
# results = powerlaw.Fit(data, discrete=True)
# print(results.power_law.alpha)
# print(results.power_law.xmin)
# R, p = results.distribution_compare('power_law', 'lognormal')

# Fit the power law distribution
fit = powerlaw.Fit(data, xmin=1)  # Specify xmin if known

# Plot the data and fit
fig = fit.plot_pdf(color='b', linewidth=2)
fit.power_law.plot_pdf(color='b', linestyle='--', ax=fig)

# Display the estimated exponent of the power law
print("Estimated alpha parameter (exponent):", fit.power_law.alpha)

# Perform statistical tests
print("Power law fit compared to normal distribution:")
print(fit.distribution_compare('power_law', 'exponential'))

# Show the plot
plt.show()



# data = np.genfromtxt('py-powerlaw-import.txt')
# fit = powerlaw.Fit(data, discrete=True)

# R,p=fit.distribution_compare('power_law', 'lognormal')
# print('Figure 4 (R,p)',R,p)

# y=fit.lognormal.ccdf()
# print('Figure 4 Lognormal CCDF fit values')
# print(y)

# fig = fit.plot_ccdf(linewidth=3, label='Empirical Data')
# fit.power_law.plot_ccdf(ax=fig, color='r', linestyle='--', label='Power law fit')
# fit.lognormal.plot_ccdf(ax=fig, color='g', linestyle='--', label='Lognormal fit')

# fig.set_ylabel(u"p(X≥x)")
# fig.set_xlabel("Word Frequency")
# handles, labels = fig.get_legend_handles_labels()
# fig.legend(handles, labels, loc=3)
# pl.ylim(1e-4,2)
# pl.show()

# figname = 'FigLognormal'
# pl.savefig(figname+'.eps', bbox_inches='tight')



####
# fit = powerlaw.Fit(data, discrete=True, estimate_discrete=False)
# print(fit.xmin)
# print(fit.alpha)
# print(fit.power_law._pdf_discrete_normalizer)
# print(fit.distribution_compare('power_law', 'lognormal'))
