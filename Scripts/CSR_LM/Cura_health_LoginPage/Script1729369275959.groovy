import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.callTestCase(findTestCase('Cura_Health_Page1'), [:], FailureHandling.STOP_ON_FAILURE)

if (WebUI.verifyElementNotChecked(findTestObject('Object Repository/katalon_FirstTest/CURA_Firstpage/Page_CURA Healthcare Service/input_Apply for hospital readmission_hospit_63901f'), 
    6)) {
    WebUI.comment('not checked now it is clicked')
}

WebUI.click(findTestObject('katalon_FirstTest/CURA_Firstpage/Page_CURA Healthcare Service/input_Apply for hospital readmission_hospit_63901f'))

WebUI.verifyElementChecked(findTestObject('Object Repository/katalon_FirstTest/CURA_Firstpage/Page_CURA Healthcare Service/input_Apply for hospital readmission_hospit_63901f'), 
    5)

WebUI.click(findTestObject('katalon_FirstTest/CURA_Firstpage/Page_CURA Healthcare Service/label_Apply for hospital readmission'))

WebUI.click(findTestObject('katalon_FirstTest/CURA_Firstpage/Page_CURA Healthcare Service/input_Medicaid_programs'))

WebUI.click(findTestObject('katalon_FirstTest/CURA_Firstpage/Page_CURA Healthcare Service/input_Medicare_programs'))

WebUI.click(findTestObject('katalon_FirstTest/CURA_Firstpage/Page_CURA Healthcare Service/input_None_programs'))

